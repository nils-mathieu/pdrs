//! Defines the global state of the application.

use std::sync::OnceLock;

/// An instance of this type is stored globally as a singleton and contains
/// all the global state of the application.
///
/// See the freestanding functions of [this module](self) for easy
/// access to the global state.
pub struct GlobalState {}

/// The global state of the application.
static STATE: OnceLock<GlobalState> = OnceLock::new();

/// Initializes the global state of the application.
pub async fn initialize() {
    STATE
        .set(GlobalState {})
        .unwrap_or_else(|_| panic!("the global state was already initialized"));
}

/// Returns a reference to the global state of the application.
#[track_caller]
pub fn get() -> &'static GlobalState {
    STATE.get().expect("the global state was not initialized")
}
