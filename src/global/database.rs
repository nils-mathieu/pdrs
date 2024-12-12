use {
    crate::{expect_env, try_get_and_parse_env, try_get_env},
    argon2::{Argon2, PasswordHash, PasswordVerifier},
    base64ct::Encoding,
    libsql::params::IntoParams,
    rand::{rngs::StdRng, RngCore, SeedableRng},
    serde::{Deserialize, Serialize},
    std::{cell::RefCell, str::FromStr},
};

/// Wraps an SQLite database object responsible for storing the application's
/// data.
pub struct Database {
    /// The inner database object.
    db: libsql::Database,
}

impl Database {
    /// Creates a new database object.
    ///
    /// # Panics
    ///
    /// This function panics if the database object cannot be created.
    pub async fn new() -> Self {
        let db = create_db_object().await;

        Self { db }
    }
}

/// Creates a new [`libsql::Database`] object using the environment variables
/// to configure it.
async fn create_db_object() -> libsql::Database {
    let database_file = expect_env("RPDS_DATABASE_FILE");

    libsql::Builder::new_local(database_file.as_str())
        .build()
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to create a database object at `{database_file}`: {err}")
        })
}
