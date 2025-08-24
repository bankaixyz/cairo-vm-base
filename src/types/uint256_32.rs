use crate::cairo_type::{BaseCairoType, CairoType};
use crate::types::{hex_bytes_padded, FromAnyStr};
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Uint256Bits32(pub BigUint);

impl BaseCairoType for Uint256Bits32 {
    fn from_bytes_be(bytes: &[u8]) -> Self {
        if bytes.len() != 32 {
            panic!("Invalid bytes length for Uint256Bits32");
        }
        Uint256Bits32(BigUint::from_bytes_be(bytes))
    }

    fn bytes_len() -> usize {
        32
    }
}

impl Uint256Bits32 {
    pub fn to_limbs(&self) -> [Felt252; 8] {
        const LIMB_SIZE: u32 = 32;
        let limb_mask = (BigUint::from(1u64) << LIMB_SIZE) - BigUint::from(1u64);

        let limbs = (0..8)
            .map(|i| {
                let shift = (7 - i) * LIMB_SIZE;
                let limb = (&self.0 >> shift) & &limb_mask;
                Felt252::from_bytes_be_slice(&limb.to_bytes_be())
            })
            .collect::<Vec<_>>();

        limbs.try_into().unwrap()
    }
}

impl CairoType for Uint256Bits32 {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError> {
        let mut bigint = BigUint::from(0u32);

        for i in (0..8).rev() {
            let value = BigUint::from_bytes_be(&vm.get_integer((address + i)?)?.to_bytes_be());
            bigint = (bigint << 32) | value;
        }

        Ok(Self(bigint))
    }

    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError> {
        let limbs = self.to_limbs();

        for (i, limb) in limbs.iter().enumerate() {
            vm.insert_value((address + i)?, *limb)?;
        }

        Ok((address + 8)?)
    }

    fn n_fields() -> usize {
        8
    }
}

impl FromAnyStr for Uint256Bits32 {
    fn from_any_str(s: &str) -> Result<Self, String> {
        if !s.starts_with("0x") && !s.starts_with("0X") {
            if let Some(value) = BigUint::parse_bytes(s.as_bytes(), 10) {
                return Ok(Uint256Bits32(value));
            }
        }
        // If it has a prefix or decimal parsing fails, treat as hex.
        let bytes = hex_bytes_padded(s, Some(32))?; // 256 bits
        Ok(Uint256Bits32(BigUint::from_bytes_be(&bytes)))
    }
}

impl<'de> serde::Deserialize<'de> for Uint256Bits32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::types::serde_utils::deserialize_from_any(deserializer)
    }
}

impl serde::Serialize for Uint256Bits32 {
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
