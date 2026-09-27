// Data formats: Serde, JSON, TOML, CSV, YAML, MessagePack

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

pub mod serde {
    use std::fmt;

    pub trait Serialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer;
    }

    pub trait Deserialize<'de>: Sized {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>;
    }

    pub trait Serializer {
        type Ok;
        type Error;
    }

    pub trait Deserializer<'de> {
        type Error;
    }
}

pub mod json {
    use super::Error;

    pub fn to_string<T: serde::Serialize>(value: &T) -> Result<String, Error> {
        Ok("{}".to_string())
    }

    pub fn from_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, Error> {
        unimplemented!()
    }
}

pub mod toml {
    use super::Error;

    pub fn to_string<T: serde::Serialize>(value: &T) -> Result<String, Error> {
        Ok("".to_string())
    }

    pub fn from_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, Error> {
        unimplemented!()
    }
}