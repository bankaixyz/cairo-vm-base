use crate::cairo_type::{BaseCairoType, CairoType};
use crate::types::{hex_bytes_padded, FromAnyStr};
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Felt(pub Felt252);

impl BaseCairoType for Felt {
    fn from_bytes_be(bytes: &[u8]) -> Self {
        if bytes.len() != 32 {
            panic!("Invalid bytes length for Felt");
        }
        Felt(Felt252::from_bytes_be_slice(bytes))
    }

    fn bytes_len() -> usize {
        32
    }
}

impl CairoType for Felt {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError> {
        let value = vm.get_integer((address + 0)?)?;
        Ok(Self(*value))
    }

    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError> {
        vm.insert_value((address + 0)?, self.0)?;
        Ok((address + 1)?)
    }

    fn n_fields() -> usize {
        1
    }
}

impl FromAnyStr for Felt {
    fn from_any_str(s: &str) -> Result<Self, String> {
        if !s.starts_with("0x") && !s.starts_with("0X") {
            if let Ok(value) = Felt252::from_dec_str(s) {
                return Ok(Felt(value));
            }
        }
        // If it has a prefix or decimal parsing fails, treat as hex.
        let bytes = hex_bytes_padded(s, None)?;
        Ok(Felt(Felt252::from_bytes_be_slice(&bytes)))
    }
}

impl<'de> serde::Deserialize<'de> for Felt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::types::serde_utils::deserialize_from_any(deserializer)
    }
}

impl serde::Serialize for Felt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.0.to_bytes_be();
        let hex = hex::encode(bytes);
        serializer.serialize_str(&format!("0x{}", hex))
    }
}
