pub mod felt;
pub mod uint256;
pub mod uint384;

// Shared hex parsing trait and helper
pub trait FromHexStr: Sized {
    fn from_hex_str(s: &str) -> Result<Self, String>;
}

pub fn from_hex<T: FromHexStr>(s: &str) -> Result<T, String> {
    T::from_hex_str(s)
}

pub fn hex_bytes_padded(input: &str, target_len: Option<usize>) -> Result<Vec<u8>, String> {
    let mut hex = input.strip_prefix("0x").or_else(|| input.strip_prefix("0X")).unwrap_or(input).to_string();
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
