use {
    super::{Did, Handle},
    serde::Serialize,
    std::fmt::Display,
};

/// An identifier that can be used to reference a repository.
#[derive(Debug, Clone)]
pub enum AtIdentifier<T = Box<str>> {
    /// The identifier is a DID.
    Did(Did<T>),
    /// The identifier is a URI.
    Handle(Handle<T>),
}

impl<T: AsRef<str>> AtIdentifier<T> {
    /// Returns the identifier as a string.
    pub fn as_str(&self) -> &str {
        match self {
            AtIdentifier::Did(did) => did.as_str(),
            AtIdentifier::Handle(handle) => handle.as_str(),
        }
    }
}

impl<T: AsRef<str>> Display for AtIdentifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtIdentifier::Did(did) => Display::fmt(did, f),
            AtIdentifier::Handle(handle) => Display::fmt(handle, f),
        }
    }
}

impl<T> Serialize for AtIdentifier<T>
where
    T: AsRef<str>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AtIdentifier::Did(did) => did.serialize(serializer),
            AtIdentifier::Handle(handle) => handle.serialize(serializer),
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for AtIdentifier<T>
where
    T: serde::Deserialize<'de> + AsRef<str>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: T = serde::Deserialize::deserialize(deserializer)?;

        if s.as_ref().starts_with("did:") {
            match Did::new(s) {
                Ok(did) => Ok(AtIdentifier::Did(did)),
                Err(e) => Err(serde::de::Error::custom(e)),
            }
        } else {
            match Handle::new(s) {
                Ok(handle) => Ok(AtIdentifier::Handle(handle)),
                Err(e) => Err(serde::de::Error::custom(e)),
            }
        }
    }
}
