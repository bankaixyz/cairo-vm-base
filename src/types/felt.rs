use crate::cairo_type::CairoType;
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use serde::{Deserialize, Serialize};
use crate::types::{FromHexStr, hex_bytes_padded};

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

impl FromHexStr for Felt {
    fn from_hex_str(s: &str) -> Result<Self, String> {
        let bytes = hex_bytes_padded(s, None)?;
        Ok(Felt(Felt252::from_bytes_be_slice(&bytes)))
    }
}
