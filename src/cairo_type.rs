use crate::types::FromAnyStr;
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
};

pub trait CairoType: Sized + FromAnyStr {
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