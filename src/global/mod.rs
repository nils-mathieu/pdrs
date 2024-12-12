//! Defines the global state of the application.

use {
    self::{database::Database, password::PasswordHasher},
    crate::expect_env,
    std::sync::OnceLock,
};

pub mod database;
pub mod password;

/// An instance of this type is stored globally as a singleton and contains
/// all the global state of the application.
///
/// See the freestanding functions of [this module](self) for easy
/// access to the global state.
pub struct GlobalState {
    /// The password hasher responsible for hashing and verifying passwords.
    pub password_hasher: PasswordHasher,
    /// The database instance.
    pub database: Database,
}

/// The global state of the application.
static STATE: OnceLock<GlobalState> = OnceLock::new();

/// Initializes the global state of the application.
pub async fn initialize() {
    let database = Database::new().await;
    let password_hasher = PasswordHasher::new();

    STATE
        .set(GlobalState {
            database,
            password_hasher,
        })
        .unwrap_or_else(|_| panic!("the global state was already initialized"));
}

/// Returns a reference to the global state of the application.
#[track_caller]
pub fn get() -> &'static GlobalState {
    STATE.get().expect("the global state was not initialized")
}
