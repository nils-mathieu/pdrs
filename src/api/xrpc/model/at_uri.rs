use {
    super::{validate_did, validate_handle},
    memchr::{memchr, memchr3},
    serde::{Deserialize, Serialize},
};

/// An error that might occur when parsing an `at://` URI from a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtUriParseError;

impl std::fmt::Display for AtUriParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("failed to parse `at://` URI")
    }
}

impl std::error::Error for AtUriParseError {}

/// Stores indices of the different parts of an `at://` URI.
#[derive(Debug, Clone, Copy)]
struct Parts {
    authority_end: u16,
    path_end: u16,
    query: (u16, u16),
    fragment_start: u16,
}

/// Represents an `at://` URI object.
#[derive(Clone, Copy)]
pub struct AtUri<T: ?Sized = Box<str>> {
    parts: Parts,
    value: T,
}

impl<T: ?Sized + AsRef<str>> AtUri<T> {
    /// Creates a new [`AtUri`] instance from the provided value.
    pub fn new(val: T) -> Result<Self, AtUriParseError>
    where
        T: Sized,
    {
        if let Some(parts) = compute_parts(val.as_ref().as_bytes()) {
            Ok(AtUri { parts, value: val })
        } else {
            Err(AtUriParseError)
        }
    }

    /// Returns the underlying string.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    /// Returns the authority part of the URI.
    pub fn authority_str(&self) -> &str {
        let start = 5;
        let end = self.parts.authority_end as usize;

        unsafe { self.value.as_ref().get_unchecked(start..end) }
    }

    /// Returns the path part of the URI.
    pub fn path(&self) -> &str {
        let start = self.parts.authority_end as usize;
        let end = self.parts.path_end as usize;

        unsafe { self.value.as_ref().get_unchecked(start..end) }
    }

    /// Returns the query part of the URI.
    pub fn query(&self) -> &str {
        let (start, end) = self.parts.query;
        let start = start as usize;
        let end = end as usize;

        unsafe { self.value.as_ref().get_unchecked(start..end) }
    }

    /// Returns the fragment part of the URI.
    pub fn fragment(&self) -> &str {
        let start = self.parts.fragment_start as usize;
        unsafe { self.value.as_ref().get_unchecked(start..) }
    }
}

impl<T: ?Sized + AsRef<str>> std::fmt::Debug for AtUri<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AtUri").field(&self.as_str()).finish()
    }
}

impl<T: ?Sized + AsRef<str>> std::fmt::Display for AtUri<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

impl<'a> TryFrom<&'a str> for AtUri<&'a str> {
    type Error = AtUriParseError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Box<str>> for AtUri<Box<str>> {
    type Error = AtUriParseError;

    #[inline]
    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for AtUri<String> {
    type Error = AtUriParseError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Validates the provided `at://` URI.
#[inline]
pub fn validate_at_uri(bytes: &[u8]) -> bool {
    compute_parts(bytes).is_some()
}

/// Computes the parts of the provided URI, validating it at the same time.
fn compute_parts(mut bytes: &[u8]) -> Option<Parts> {
    let og_ptr = bytes.as_ptr();

    // The URI must not exceed 8 KB.
    if bytes.len() >= 8 * 1024 {
        return None;
    }

    bytes = parse_at_prefix(bytes)?;

    let authority;
    (authority, bytes) = split_path_component(bytes);

    if !validate_authority(authority) {
        return None;
    }

    // SAFETY: Both those pointers are part of the same allocation.
    let authority_end = unsafe { bytes.as_ptr().offset_from(og_ptr) } as u16;

    loop {
        let end_of_previous_component = unsafe { bytes.as_ptr().offset_from(og_ptr) } as u16;

        let token;
        (token, bytes) = match bytes.split_first() {
            Some(some) => some,

            // We exhausted the input string.
            None => {
                return Some(Parts {
                    authority_end,
                    path_end: end_of_previous_component,
                    query: (0, 0),
                    fragment_start: end_of_previous_component,
                });
            }
        };

        match token {
            b'/' => {
                let component;
                (component, bytes) = split_path_component(bytes);

                if !validate_path_component(component) {
                    return None;
                }
            }
            b'#' => {
                if !validate_fragment(bytes) {
                    return None;
                }

                let fragment_start = unsafe { bytes.as_ptr().offset_from(og_ptr) } as u16;

                return Some(Parts {
                    authority_end,
                    path_end: end_of_previous_component,
                    query: (0, 0),
                    fragment_start,
                });
            }
            b'?' => {
                let index = memchr(b'#', bytes).unwrap_or(bytes.len());

                // SAFETY: `memchr` returned a valid index.
                let query = unsafe { bytes.get_unchecked(..index) };

                let query_start = unsafe { query.as_ptr().offset_from(og_ptr) } as u16;
                let query_end = query_start + query.len() as u16;

                let fragment_start = if index < bytes.len() {
                    // SAFETY: We just made sure that `index < bytes.len()`.
                    let fragment = unsafe { bytes.get_unchecked(index + 1..) };

                    if !validate_fragment(fragment) {
                        return None;
                    }

                    unsafe { fragment.as_ptr().offset_from(og_ptr) as u16 }
                } else {
                    query_end
                };

                if !validate_query(query) {
                    return None;
                }

                return Some(Parts {
                    authority_end,
                    path_end: end_of_previous_component,
                    query: (query_start, query_end),
                    fragment_start,
                });
            }
            _ => unreachable!(),
        }
    }
}

/// Returns whether the provided byte is a valid URI character.
#[inline]
fn is_valid_uri_character(c: &u8) -> bool {
    matches!(c, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_' | b'~')
}

/// Validates the provided bytes as a path component.
fn validate_path_component(bytes: &[u8]) -> bool {
    bytes.iter().all(is_valid_uri_character)
}

/// Validates the provided bytes as an URI fragment.
fn validate_fragment(bytes: &[u8]) -> bool {
    bytes.iter().all(is_valid_uri_character)
}

/// Validates the provided bytes as an URI query.
fn validate_query(bytes: &[u8]) -> bool {
    bytes.iter().all(is_valid_uri_character)
}

/// Parses the provided bytes to extract the initial `at://` prefix.
///
/// If the prefix is found, the function returns the remaining bytes. Otherwise,
/// it returns `None`.
fn parse_at_prefix(mut bytes: &[u8]) -> Option<&[u8]> {
    let scheme;
    (scheme, bytes) = bytes.split_at_checked(5)?;

    if scheme != b"at://" {
        return None;
    }

    Some(bytes)
}

/// Splits the provided string into a path component and the remaining bytes.
///
/// Note that the splitting character (`/`, `?`, etc.) is included
/// in the second slice.
fn split_path_component(bytes: &[u8]) -> (&[u8], &[u8]) {
    let index = memchr3(b'/', b'?', b'#', bytes).unwrap_or(bytes.len());

    // SAFETY: `memchr3` returned a valid index, and `bytes.len()` is a valid
    // index for both those operations.
    let first = unsafe { bytes.get_unchecked(..index) };
    let second = unsafe { bytes.get_unchecked(index..) };

    (first, second)
}

/// Validates the provided `authority` string.
fn validate_authority(authority: &[u8]) -> bool {
    if authority.is_empty() {
        return false;
    }

    if authority.starts_with(b"did:") && validate_did(authority) {
        return true;
    }

    if validate_handle(authority) {
        return true;
    }

    false
}

impl<T: ?Sized + AsRef<str>> Serialize for AtUri<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de, T: AsRef<str> + Deserialize<'de>> Deserialize<'de> for AtUri<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = T::deserialize(deserializer)?;
        Self::new(inner).map_err(serde::de::Error::custom)
    }
}

#[test]
#[cfg(test)]
fn authority_only_did() {
    let uri = AtUri::new("at://did:example:123").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "");
    assert_eq!(uri.query(), "");
    assert_eq!(uri.fragment(), "");
}

#[test]
#[cfg(test)]
fn authority_and_path() {
    let uri = AtUri::new("at://did:example:123/hello").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "/hello");
    assert_eq!(uri.query(), "");
    assert_eq!(uri.fragment(), "");
}

#[test]
#[cfg(test)]
fn authority_and_empty_path() {
    let uri = AtUri::new("at://did:example:123/").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "/");
    assert_eq!(uri.query(), "");
    assert_eq!(uri.fragment(), "");
}

#[test]
#[cfg(test)]
fn all_parts() {
    let uri = AtUri::new("at://did:example:123/hello?query#fragment").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "/hello");
    assert_eq!(uri.query(), "query");
    assert_eq!(uri.fragment(), "fragment");
}

#[test]
#[cfg(test)]
fn all_parts_no_query() {
    let uri = AtUri::new("at://did:example:123/hello#fragment").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "/hello");
    assert_eq!(uri.query(), "");
    assert_eq!(uri.fragment(), "fragment");
}

#[test]
#[cfg(test)]
fn long_path() {
    let uri = AtUri::new("at://did:example:123/test/test/test/").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "/test/test/test/");
    assert_eq!(uri.query(), "");
    assert_eq!(uri.fragment(), "");
}

#[test]
#[cfg(test)]
fn long_path_no_slash_terminator() {
    let uri = AtUri::new("at://did:example:123/test/test/test").unwrap();
    assert_eq!(uri.authority_str(), "did:example:123");
    assert_eq!(uri.path(), "/test/test/test");
    assert_eq!(uri.query(), "");
    assert_eq!(uri.fragment(), "");
}
