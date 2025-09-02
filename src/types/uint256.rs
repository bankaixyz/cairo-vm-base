use crate::cairo_type::{BaseCairoType, CairoType};
use crate::types::{hex_bytes_padded, FromAnyStr};
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Uint256(pub BigUint);

impl BaseCairoType for Uint256 {
    fn from_bytes_be(bytes: &[u8]) -> Self {
        if bytes.len() > 32 {
            panic!(
                "Invalid bytes length for Uint256. Expected 32 bytes, got {}",
                bytes.len()
            );
        }
        Uint256(BigUint::from_bytes_be(bytes))
    }

    fn bytes_len() -> usize {
        32
    }
}

impl Uint256 {
    pub fn to_limbs(&self) -> [Felt252; 2] {
        const LIMB_SIZE: u32 = 128;
        let limb_mask = (BigUint::from(1u128) << LIMB_SIZE) - BigUint::from(1u128);

        let lower_limb = &self.0 & &limb_mask;
        let upper_limb = &self.0 >> LIMB_SIZE;

        [
            Felt252::from_bytes_be_slice(&lower_limb.to_bytes_be()),
            Felt252::from_bytes_be_slice(&upper_limb.to_bytes_be()),
        ]
    }
}

impl CairoType for Uint256 {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError> {
        let d0 = BigUint::from_bytes_be(&vm.get_integer((address + 0)?)?.to_bytes_be());
        let d1 = BigUint::from_bytes_be(&vm.get_integer((address + 1)?)?.to_bytes_be());
        let bigint = d1 << 128 | d0;
        Ok(Self(bigint))
    }

    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError> {
        let limbs = self.to_limbs();
        println!("limbs: {:?}", limbs);
        vm.insert_value((address + 0)?, limbs[0])?;
        vm.insert_value((address + 1)?, limbs[1])?;
        Ok((address + 2)?)
    }

    fn n_fields() -> usize {
        2
    }
}

impl FromAnyStr for Uint256 {
    fn from_any_str(s: &str) -> Result<Self, String> {
        if !s.starts_with("0x") && !s.starts_with("0X") {
            if let Some(value) = BigUint::parse_bytes(s.as_bytes(), 10) {
                return Ok(Uint256(value));
            }
        }
        // If it has a prefix or decimal parsing fails, treat as hex.
        let bytes = hex_bytes_padded(s, Some(32))?; // 256 bits
        Ok(Uint256(BigUint::from_bytes_be(&bytes)))
    }
}

impl<'de> serde::Deserialize<'de> for Uint256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::types::serde_utils::deserialize_from_any(deserializer)
    }
}

impl serde::Serialize for Uint256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.0.to_bytes_be();
        let mut padded_bytes = vec![0u8; 32]; // 256 bits = 32 bytes
        let start = 32 - bytes.len();
        padded_bytes[start..].copy_from_slice(&bytes);
        let hex = hex::encode(padded_bytes);
        serializer.serialize_str(&format!("0x{}", hex))
    }
}
