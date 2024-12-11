#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use {
    std::{convert::Infallible, ffi::OsString, net::SocketAddr},
    tokio::net::TcpStream,
    tracing::{error, trace},
};

mod api;
mod panic;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::level_filters::LevelFilter::TRACE)
        .init();
    std::panic::set_hook(Box::new(self::panic::panic_hook));
    run_server().await;
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
    let s = expect_env("RPDS_HOSTNAME");
    let s = s
        .to_str()
        .unwrap_or_else(|| panic!("`RPDS_HOSTNAME` is not valid UTF-8"));
    s.parse()
        .unwrap_or_else(|_| panic!("`RPDS_HOSTNAME` is not a valid socket address"))
}

/// Returns the value of the provided environment variable, or panics if it is
/// missing.
#[track_caller]
fn expect_env(env: &str) -> OsString {
    match std::env::var_os(env) {
        Some(value) => value,
        None => panic!("Environment variable `{env}` is missing"),
    }
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
