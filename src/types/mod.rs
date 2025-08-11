pub mod felt;
pub mod uint256;
pub mod uint384;

// Shared string parsing trait and helper
pub trait FromAnyStr: Sized {
    fn from_any_str(s: &str) -> Result<Self, String>;
}

pub fn from_string<T: FromAnyStr>(s: &str) -> Result<T, String> {
    T::from_any_str(s)
}

pub fn hex_bytes_padded(input: &str, target_len: Option<usize>) -> Result<Vec<u8>, String> {
    let mut hex = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
        .unwrap_or(input)
        .to_string();
    hex.retain(|c| c != '_');
    if hex.len() % 2 == 1 {
        hex.insert(0, '0');
    }
    let mut bytes = hex::decode(&hex).map_err(|e| e.to_string())?;
    if let Some(t) = target_len {
        if bytes.len() > t {
            return Err("hex value does not fit in target type".to_string());
        }
        if bytes.len() < t {
            let mut padded = vec![0u8; t - bytes.len()];
            padded.extend_from_slice(&bytes);
            bytes = padded;
        }
    }
    Ok(bytes)
}

#[cfg(feature = "serde")]
pub mod serde_utils {
    //! Serde helpers for deserializing types that implement `FromAnyStr`.

    use super::FromAnyStr;
    use serde::de::{self, Deserializer, Visitor};
    use serde::Deserialize;
    use std::fmt;

    /// Deserialize any type implementing FromAnyStr from a JSON string
    pub fn deserialize_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromAnyStr,
    {
        let s = String::deserialize(deserializer)?;
        T::from_any_str(&s).map_err(de::Error::custom)
    }

    /// Deserialize a vector of types implementing FromAnyStr from JSON string array
    pub fn deserialize_vec_from_string<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromAnyStr,
    {
        let ss: Vec<String> = Vec::deserialize(deserializer)?;
        ss.into_iter()
            .map(|s| T::from_any_str(&s))
            .collect::<Result<Vec<T>, _>>()
            .map_err(de::Error::custom)
    }

    struct AnyStrVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> Visitor<'de> for AnyStrVisitor<T>
    where
        T: FromAnyStr,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or an integer")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            T::from_any_str(value).map_err(de::Error::custom)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            T::from_any_str(&value.to_string()).map_err(de::Error::custom)
        }
    }

    /// Deserialize any type implementing FromAnyStr from either a JSON string or number
    pub fn deserialize_from_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromAnyStr,
    {
        deserializer.deserialize_any(AnyStrVisitor(std::marker::PhantomData))
    }
}
