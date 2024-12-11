use {
    memchr::memchr,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    std::fmt::Display,
};

/// Errors that can occur when parsing a DID from a string.
#[derive(Debug, Clone)]
pub struct DidParseError;

impl std::fmt::Display for DidParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid DID format")
    }
}

impl std::error::Error for DidParseError {}

/// Represents a Decentralized IDentifier (DID) as defined in the
///
/// [W3C DID specification](https://www.w3.org/TR/did-core/).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Did<T: ?Sized = Box<str>>(T);

impl<T> Did<T> {
    /// Creates a new [`Did`] instance from the provided value without
    /// validating it.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided value is a valid DID.
    #[inline]
    pub const unsafe fn new_unchecked(val: T) -> Self {
        Did(val)
    }
}

impl<T> Did<T>
where
    T: ?Sized + AsRef<str>,
{
    /// Creates a new [`Did`] instance from the provided value.
    ///
    /// If the value is not a valid DID, this function fails.
    pub fn new(val: T) -> Result<Self, DidParseError>
    where
        T: Sized,
    {
        if validate_did(val.as_ref().as_bytes()) {
            Ok(unsafe { Did::new_unchecked(val) })
        } else {
            Err(DidParseError)
        }
    }

    /// Returns the underlying byte slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: ?Sized + AsRef<str>> Display for Did<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

/// Validates the provided `bytes` string as a DID.
pub fn validate_did(mut bytes: &[u8]) -> bool {
    let prefix;
    (prefix, bytes) = match bytes.split_at_checked(4) {
        Some(some) => some,
        None => return false,
    };

    if prefix != b"did:" {
        return false;
    }

    let Some(method_end_index) = memchr(b':', bytes) else {
        return false;
    };
    if method_end_index == 0 {
        return false;
    }

    // SAFETY: The `memchr` function returned a valid index, which ensures
    // that `method_end_index` is within the bounds of `bytes`.
    let method = unsafe { bytes.get_unchecked(..method_end_index) };
    bytes = unsafe { bytes.get_unchecked(method_end_index..) };

    #[inline]
    fn is_method_char(c: &u8) -> bool {
        matches!(c, 0x61..=0x7A | b'0'..=b'9')
    }

    if !method.iter().all(is_method_char) {
        return false;
    }

    #[inline]
    fn is_id_char(c: u8) -> bool {
        matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'_' | b':')
    }

    let mut i = 0;
    while i < bytes.len() {
        // SAFETY: We just made sure that `i` is in bounds in the loop
        // condition.
        let c = unsafe { *bytes.get_unchecked(i) };

        if c == b'%' {
            // The next two characters must be hexadecimal digits.

            if i + 2 >= bytes.len() {
                return false;
            }

            // SAFETY: We just checked that `i + 2` is within bounds.
            let a = unsafe { *bytes.get_unchecked(i + 1) };
            let b = unsafe { *bytes.get_unchecked(i + 2) };

            if !a.is_ascii_hexdigit() || !b.is_ascii_hexdigit() {
                return false;
            }

            i += 3;
            continue;
        }

        if !is_id_char(c) {
            return false;
        }

        i += 1;
    }

    // Can't end with a colon
    if bytes.last() == Some(&b':') {
        return false;
    }

    true
}

impl<T> Serialize for Did<T>
where
    T: ?Sized + AsRef<str>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Did<T>
where
    T: Deserialize<'de> + AsRef<str>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
            .and_then(|inner| Self::new(inner).map_err(serde::de::Error::custom))
    }
}

impl<T> AsRef<str> for Did<T>
where
    T: ?Sized + AsRef<str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> TryFrom<&'a str> for Did<&'a str> {
    type Error = DidParseError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Box<str>> for Did<Box<str>> {
    type Error = DidParseError;

    #[inline]
    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Did<String> {
    type Error = DidParseError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
#[test]
fn did_with_empty_last_component() {
    assert!(!validate_did(b"did:example:test:"));
}

#[cfg(test)]
#[test]
fn did_with_consecutive_colons() {
    assert!(!validate_did(b"did:example::::test"));
}

#[cfg(test)]
#[test]
fn basic_did() {
    assert!(validate_did(b"did:example:test:awdac:sdfsdfw"));
}
