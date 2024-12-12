use {
    crate::{try_get_and_parse_env, try_get_env},
    base64ct::Encoding,
    rand::{rngs::StdRng, RngCore, SeedableRng},
    serde::{Deserialize, Serialize},
    std::cell::RefCell,
    tracing::error,
};

/// An error that can occur when hashing a password.
#[derive(Debug)]
pub struct FailedToHashPassword;

impl std::fmt::Display for FailedToHashPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to hash the password")
    }
}

impl std::error::Error for FailedToHashPassword {}

thread_local! {
    /// A pseudo-random number generator that can be used to generate random
    /// data.
    static PRNG: RefCell<StdRng> = RefCell::new(StdRng::from_entropy());
}

const ARGON2_VERSION: argon2::Version = argon2::Version::V0x13;
const ARGON2_ALGORITHM: argon2::Algorithm = argon2::Algorithm::Argon2id;

/// The type used to encode and decode base64-based objects.
type B64 = base64ct::Base64Unpadded;

/// Defines a collection of parameters to use when hashing new passwords.
pub struct PasswordHasher {
    /// The p-cost of the Argon2 algorithm.
    parallelism: u32,
    /// The m-cost of the Argon2 algorithm.
    memory_cost: u32,
    /// The t-cost of the Argon2 algorithm.
    time_cost: u32,
    /// The secret key used to hash passwords.
    secret: Option<&'static [u8]>,
}

impl PasswordHasher {
    /// Creates a new [`PasswordHasher`] object using the environment variables
    /// to configure it.
    pub fn new() -> Self {
        let memory_cost =
            try_get_and_parse_env("RPDS_ARGON2_MCOST").unwrap_or(argon2::Params::DEFAULT_M_COST);
        let time_cost =
            try_get_and_parse_env("RPDS_ARGON2_TCOST").unwrap_or(argon2::Params::DEFAULT_T_COST);
        let parallelism = try_get_and_parse_env("RPDS_ARGON2_PARALLELISM")
            .unwrap_or(argon2::Params::DEFAULT_P_COST);

        // Check the parameters once here to avoid crashing later during
        // password hashing.
        argon2::Params::new(memory_cost, time_cost, parallelism, None)
            .unwrap_or_else(|err| panic!("Invalid Argon2 parameters: {err}"));

        let secret = try_get_env("RPDS_ARGON2_SECRET").map(|s| s.leak().as_bytes());

        Self {
            parallelism,
            memory_cost,
            time_cost,
            secret,
        }
    }

    /// Hashes the provided password.
    pub fn hash_password(&self, password: &[u8]) -> String {
        // Do we need to run the algorithm on a separate thread to avoid
        // blocking all tasks running on the current one? I'm not sure how
        // expensive `argon2` really is.

        let argon2 = make_argon2_engine(
            ARGON2_ALGORITHM,
            ARGON2_VERSION,
            self.memory_cost,
            self.time_cost,
            self.parallelism,
            self.secret,
        );

        let mut salt = [0u8; argon2::RECOMMENDED_SALT_LEN];
        PRNG.with_borrow_mut(|rng| rng.fill_bytes(&mut salt));

        let mut output_hash = [0u8; argon2::Params::DEFAULT_OUTPUT_LEN];
        argon2
            .hash_password_into(password, &salt, &mut output_hash)
            .unwrap();

        // Encode the salt in base64 before saving it.
        let mut encoded_salt = [0u8; base64_encoded_len(argon2::RECOMMENDED_SALT_LEN).unwrap()];
        B64::encode(&salt, &mut encoded_salt).unwrap();
        // SAFETY: Base-64 encoding is guaranteed to produce a valid UTF-8.
        let salt = unsafe { std::str::from_utf8_unchecked(&encoded_salt) };

        let mut encoded_hash =
            [0u8; base64_encoded_len(argon2::Params::DEFAULT_OUTPUT_LEN).unwrap()];
        B64::encode(&output_hash, &mut encoded_hash).unwrap();
        // SAFETY: Base-64 encoding is guaranteed to produce a valid UTF-8.
        let hash = unsafe { std::str::from_utf8_unchecked(&encoded_hash) };

        let ph = SavedPasswordHash::Argon2 {
            salt,
            hash,
            p_cost: self.parallelism,
            m_cost: self.memory_cost,
            t_cost: self.time_cost,
            version: ARGON2_VERSION.into(),
            algorithm: ARGON2_ALGORITHM.ident().as_str(),
        };

        ph.serialize()
    }

    /// Verifies whether the provided password matches the saved hash.
    ///
    /// # Errors
    ///
    /// On error, this function returns `false` and an error is logged
    /// to the logging system.
    pub fn verify_password(&self, password: &[u8], hash_string: &str) -> bool {
        let saved_password_hash = match SavedPasswordHash::deserialize(hash_string) {
            Ok(s) => s,
            Err(err) => {
                error!("Failed to deserialize the saved password hash: {err}");
                return false;
            }
        };

        match saved_password_hash {
            SavedPasswordHash::Argon2 {
                salt,
                hash,
                p_cost,
                m_cost,
                t_cost,
                version,
                algorithm,
            } => {
                let algorithm = match algorithm {
                    "argon2id" => argon2::Algorithm::Argon2id,
                    "argon2i" => argon2::Algorithm::Argon2i,
                    "argon2d" => argon2::Algorithm::Argon2d,
                    _ => {
                        error!("Unknown Argon2 algorithm: {algorithm}");
                        return false;
                    }
                };

                let version = match version {
                    0x13 => argon2::Version::V0x13,
                    0x10 => argon2::Version::V0x10,
                    _ => {
                        error!("Unknown Argon2 version: {version}");
                        return false;
                    }
                };

                let argon2 =
                    make_argon2_engine(algorithm, version, m_cost, t_cost, p_cost, self.secret);

                let mut decoded_salt = [0u8; argon2::MAX_SALT_LEN];
                let salt = B64::decode(salt, &mut decoded_salt).unwrap();

                let expected_hash = B64::decode_vec(hash).unwrap();
                let mut computed_hash = vec![0u8; expected_hash.len()];

                if let Err(err) = argon2.hash_password_into(password, salt, &mut computed_hash) {
                    error!("Failed to hash the password: {err}");
                    return false;
                };

                expected_hash == computed_hash
            }
        }
    }
}

/// Creates a new Argon2 engine with the provided parameters.
///
/// # Panics
///
/// This function panics if the provided parameters are invalid
fn make_argon2_engine(
    algorithm: argon2::Algorithm,
    version: argon2::Version,
    memory_cost: u32,
    time_cost: u32,
    parallelism: u32,
    secret: Option<&[u8]>,
) -> argon2::Argon2 {
    #[inline(never)]
    #[cold]
    #[track_caller]
    fn cant_create_argon2(err: argon2::Error) -> ! {
        panic!("Failed to create an Argon2 engine: {err}")
    }

    if let Some(secret) = secret {
        argon2::Argon2::new_with_secret(
            secret,
            algorithm,
            version,
            argon2::Params::new(memory_cost, time_cost, parallelism, None)
                .unwrap_or_else(|err| cant_create_argon2(err)),
        )
        .unwrap_or_else(|err| cant_create_argon2(err))
    } else {
        argon2::Argon2::new(
            algorithm,
            version,
            argon2::Params::new(memory_cost, time_cost, parallelism, None)
                .unwrap_or_else(|err| cant_create_argon2(err)),
        )
    }
}

/// The data saved in the database to remember the parameters that were
/// used to hash a password.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "family")]
enum SavedPasswordHash<'a> {
    /// The password was hashed using the Argon2 algorithm and with the following
    /// parameters.
    Argon2 {
        #[serde(borrow)]
        salt: &'a str,
        #[serde(borrow)]
        hash: &'a str,
        p_cost: u32,
        m_cost: u32,
        t_cost: u32,
        version: u32,
        algorithm: &'a str,
    },
}

impl<'a> SavedPasswordHash<'a> {
    /// Serializes this [`SavedPasswordHash`] object into a JSON string.
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Deserializes a JSON string into a [`SavedPasswordHash`] object.
    pub fn deserialize(json: &'a str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

/// Computes the length of a base64-encoded string that contains `n` bytes.
///
/// Unpadded-style encoding is used.
const fn base64_encoded_len(n: usize) -> Option<usize> {
    match n.checked_mul(4) {
        Some(q) => Some((q / 3) + (q % 3 != 0) as usize),
        None => None,
    }
}

#[cfg(test)]
const DEFAULT_CONFIG: PasswordHasher = PasswordHasher {
    parallelism: argon2::Params::DEFAULT_P_COST,
    memory_cost: argon2::Params::DEFAULT_M_COST,
    time_cost: argon2::Params::DEFAULT_T_COST,
    secret: None,
};

#[cfg(test)]
#[test]
fn test_password_success() {
    let config = DEFAULT_CONFIG;
    let password = b"password";

    let hash = config.hash_password(password);
    assert!(config.verify_password(password, &hash));
}

#[cfg(test)]
#[test]
fn test_password_failure() {
    let config = DEFAULT_CONFIG;
    let password1 = b"testfsdf";
    let password2 = b"sdkfjcce";

    let hash = config.hash_password(password1);

    assert!(!config.verify_password(password2, &hash));
}

#[cfg(test)]
#[test]
fn test_empty_password() {
    let config = DEFAULT_CONFIG;
    let password = b"";

    let hash = config.hash_password(password);
    assert!(config.verify_password(password, &hash));
}
