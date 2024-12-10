use std::{any::Any, panic::Location};

/// Traces the provided payload to the logging system.
pub fn trace_payload(payload: &dyn Any, location: Option<&Location>) {
    let location = match location {
        Some(location) => format!(" ({location})"),
        None => "".to_owned(),
    };

    if let Some(s) = payload.downcast_ref::<&str>() {
        tracing::error!("{s}{location}");
    } else if let Some(s) = payload.downcast_ref::<String>() {
        tracing::error!("{s}{location}");
    } else {
        tracing::error!("<unknown>{location}");
    }
}

/// A panic hook that logs the panic to the logging system.
pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    trace_payload(info.payload(), info.location());
}
