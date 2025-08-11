use crate::cairo_type::CairoType;
use crate::types::{hex_bytes_padded, FromAnyStr};
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Felt(pub Felt252);

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
