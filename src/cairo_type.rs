use crate::types::FromAnyStr;
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
};

pub trait BaseCairoType: FromAnyStr + Sized + CairoType {
    fn from_bytes_be(bytes: &[u8]) -> Self;
    fn bytes_len() -> usize;
}

pub trait CairoType: Sized {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError>;
    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError>;
    fn n_fields() -> usize;
}

pub trait CairoWritable: Sized {
    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError>;
    fn n_fields() -> usize;
}
