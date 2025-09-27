use crate::cairo_type::CairoWritable;
use crate::types::{hex_bytes_padded, FromAnyStr};
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeccakBytes(pub Vec<u8>);

// Le 64 bit chunks of a byte vec for efficient keccak hash computation in cairo
impl KeccakBytes {
    pub fn to_limbs(&self) -> Vec<Felt252> {
        let mut result: Vec<Felt252> = Vec::with_capacity(self.0.len().div_ceil(8));
        for chunk in self.0.chunks(8) {
            let mut buf = [0u8; 8];
            // Copy chunk bytes as-is; interpret as little-endian u64
            for (i, b) in chunk.iter().enumerate() {
                buf[i] = *b;
            }
            let value = u64::from_le_bytes(buf);
            result.push(Felt252::from(value));
        }
        result
    }
}

impl CairoWritable for KeccakBytes {
    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError> {
        let limbs_segment = vm.add_memory_segment();

        // Write the 8 limbs to the new segment
        let limbs = self.to_limbs();
        for (i, limb) in limbs.iter().enumerate() {
            vm.insert_value((limbs_segment + i)?, *limb)?;
        }

        // Store a pointer to the new segment at the original address
        vm.insert_value(address, limbs_segment)?;

        // Return the address after the pointer
        Ok((address + 1)?)
    }

    fn n_fields() -> usize {
        1
    }
}

impl FromAnyStr for KeccakBytes {
    fn from_any_str(s: &str) -> Result<Self, String> {
        let hex_decoded = hex_bytes_padded(s, None)?;
        Ok(KeccakBytes(hex_decoded.clone()))
    }
}

impl<'de> serde::Deserialize<'de> for KeccakBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        KeccakBytes::from_any_str(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for KeccakBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let hex = hex::encode(self.0.clone());
        serializer.serialize_str(&format!("0x{hex}"))
    }
}
