pub mod felt;
pub mod keccak_bytes;
pub mod uint256;
pub mod uint256_32;
pub mod uint384;

#[cfg(test)]
mod tests;

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

pub mod serde_utils {
    //! Serde helpers for deserializing types that implement `FromAnyStr`.

    use super::FromAnyStr;
    use serde::de::{self, Deserializer, Visitor};
    use serde::Deserialize;
    use std::fmt;

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

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value < 0 {
                return Err(de::Error::custom("negative values not supported"));
            }
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

    /// Deserialize a vector of types that have custom Deserialize implementations
    /// This works with any type T that implements Deserialize, including our Cairo types
    pub fn deserialize_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Vec::<T>::deserialize(deserializer)
    }
}
