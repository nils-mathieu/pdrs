#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use {
    std::{convert::Infallible, ffi::OsString, net::SocketAddr, str::FromStr, time::Duration},
    tokio::net::TcpStream,
    tracing::{error, info, trace, warn},
};

mod api;
mod global;
mod panic;

/// The glorious entry point.
fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::level_filters::LevelFilter::TRACE)
        .init();
    std::panic::set_hook(Box::new(self::panic::panic_hook));

    if let Err(err) = dotenvy::dotenv() {
        if !err.not_found() {
            error!("Failed to load the `.env` file: {err}");
        }
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|err| panic!("failed to initialize the async runtime: {err}"));

    rt.block_on(self::global::initialize());
    rt.block_on(self::main_async());

    if rt.metrics().num_alive_tasks() > 0 {
        info!("There are still tasks running. Waiting at most 30 seconds for them to finish.");
    }

    rt.shutdown_timeout(Duration::from_secs(30));
}

/// The entry point of the async runtime.
async fn main_async() {
    tokio::select!(
        _ = run_server() => (),
        _ = wait_for_shutdown_signal() => (),
    );
}

/// A function that completes when the shutdown signal has been received.
async fn wait_for_shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received the CTRL+C signal. Shutting down the server.");
        }
        Err(_err) => {
            warn!("Failed to wait for the CTRL+C signal. Using it won't gracefully shut the server down.");
            std::future::pending().await
        }
    }
}

/// Runs the server to completion.
///
/// # Panics
///
/// This function panics on any error during initialization. This specifically
/// includes missing environment variables, or OS errors.
async fn run_server() {
    let hostname = get_socket_address();
    let listener = tokio::net::TcpListener::bind(hostname)
        .await
        .unwrap_or_else(|err| panic!("Can't bind to `{hostname}`: {err}"));

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                trace!("Accepted connection with `{addr}`");
                tokio::spawn(handle_connection(stream, addr));
            }
            Err(err) => {
                error!("Failed to accept connection: {err}");
            }
        }
    }
}

/// Parses the provided socket address, which is expected to be a hostname.
/// Panics if the provided hostname is invalid.
#[track_caller]
fn get_socket_address() -> SocketAddr {
    expect_env("RPDS_HOSTNAME")
        .parse()
        .unwrap_or_else(|_| panic!("`RPDS_HOSTNAME` is not a valid socket address"))
}

/// Returns the value of the provided environment variable, or panics if it is
/// missing.
#[track_caller]
fn expect_env(env: &str) -> String {
    match dotenvy::var(env) {
        Ok(value) => value,
        Err(err) => panic!("Environment variable `{env}`: {err}"),
    }
}

/// Returns the value of the provided environment variable, or `None` if it is
/// missing.
fn try_get_env(env: &str) -> Option<String> {
    match dotenvy::var(env) {
        Ok(value) => Some(value),
        Err(dotenvy::Error::EnvVar(std::env::VarError::NotPresent)) => None,
        Err(err) => panic!("Environment variable `{env}`: {err}"),
    }
}

/// Returns the value of the provided environment variable.
fn try_get_and_parse_env<T>(env: &str) -> Option<T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    try_get_env(env).map(|val| {
        val.parse()
            .unwrap_or_else(|err| panic!("Failed to parse `{env}`: {err}"))
    })
}

/// Handles a connection from a client.
async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    let executor = hyper_util::rt::TokioExecutor::new();
    let stream = hyper_util::rt::TokioIo::new(stream);

    let service = hyper::service::service_fn(move |mut req| async move {
        req.extensions_mut().insert(addr);
        Ok::<_, Infallible>(self::api::handle_request(&mut req).await)
    });

    if let Err(err) = hyper_util::server::conn::auto::Builder::new(executor)
        .serve_connection_with_upgrades(stream, service)
        .await
    {
        error!("Failed to serve the request: {err}");
    }
}
