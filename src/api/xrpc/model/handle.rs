use {
    memchr::{memchr, memrchr},
    serde::{Deserialize, Serialize, Serializer},
};

/// An error that might occur when parsing a handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandleParseError;

impl std::fmt::Display for HandleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("failed to parse handle")
    }
}

impl std::error::Error for HandleParseError {}

/// Represents a valid handle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle<T: ?Sized = Box<str>>(pub T);

impl<T> Handle<T> {
    /// Creates a new [`Handle`] instance from the provided value without
    /// validating it.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided value is a valid handle.
    #[inline]
    pub const unsafe fn new_unchecked(val: T) -> Self {
        Handle(val)
    }
}

impl<T> Handle<T>
where
    T: ?Sized + AsRef<str>,
{
    /// Creates a new [`Handle`] instance from the provided value.
    ///
    /// If the value is not a valid handle, this function fails.
    pub fn new(val: T) -> Result<Self, HandleParseError>
    where
        T: Sized,
    {
        if validate_handle(val.as_ref().as_bytes()) {
            Ok(unsafe { Handle::new_unchecked(val) })
        } else {
            Err(HandleParseError)
        }
    }

    /// Returns the underlying byte slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<&'a str> for Handle<&'a str> {
    type Error = HandleParseError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Box<str>> for Handle<Box<str>> {
    type Error = HandleParseError;

    #[inline]
    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Handle<String> {
    type Error = HandleParseError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T: ?Sized + AsRef<str>> std::fmt::Display for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.0.as_ref())
    }
}

/// Validates the provided handle.
pub fn validate_handle(handle: &[u8]) -> bool {
    if handle.len() > 253 {
        return false;
    }

    let last_dot = match memrchr(b'.', handle) {
        Some(pos) => pos,
        None => return validate_tld(handle),
    };

    // SAFETY: `memrchr` returns a valid index.
    let mut rest = unsafe { handle.get_unchecked(..last_dot) };

    loop {
        let index = memchr(b'.', rest).unwrap_or(rest.len());

        let label = unsafe { rest.get_unchecked(..index) };
        rest = unsafe { rest.get_unchecked((index + 1)..) };

        if !validate_label(label) {
            return false;
        }
    }
}

fn validate_tld(tld: &[u8]) -> bool {
    let (first, rest) = match tld.split_first() {
        Some(some) => some,
        None => return false,
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    rest.iter()
        .all(|&c| matches!(c, b'a'..=b'z' | b'0'..=b'9' | b'-'))
}

fn validate_label(label: &[u8]) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }

    if let [unique] = label {
        return unique.is_ascii_lowercase();
    }

    let first = unsafe { *label.get_unchecked(0) };
    let last = unsafe { *label.get_unchecked(label.len() - 1) };
    let rest = unsafe { label.get_unchecked(1..(label.len() - 1)) };

    if !first.is_ascii_lowercase() || !last.is_ascii_lowercase() {
        return false;
    }

    rest.iter()
        .all(|&c| matches!(c, b'a'..=b'z' | b'0'..=b'9' | b'-' ))
}

impl<T: ?Sized + AsRef<str>> Serialize for Handle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Handle<T>
where
    T: AsRef<str> + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer)
            .and_then(|val| Handle::new(val).map_err(serde::de::Error::custom))
    }
}
