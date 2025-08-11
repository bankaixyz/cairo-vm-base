use cairo_vm::{
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::HintProcessorData,
    types::exec_scope::ExecutionScopes,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use std::collections::HashMap;

pub mod debug;
pub mod sha256;
pub mod utils;

pub type HintImpl = fn(
    &mut VirtualMachine,
    &mut ExecutionScopes,
    &HintProcessorData,
    &HashMap<String, Felt252>,
) -> Result<(), HintError>;

pub fn default_hint_mapping() -> HashMap<String, HintImpl> {
    let mut hints = HashMap::<String, HintImpl>::new();
    hints.insert(
        sha256::HINT_SHA256_FINALIZE.into(),
        sha256::hint_sha256_finalize,
    );
    hints.insert(debug::PRINT_FELT_HEX.into(), debug::print_felt_hex);
    hints.insert(debug::PRINT_FELT.into(), debug::print_felt);
    hints.insert(debug::PRINT_STRING.into(), debug::print_string);
    hints.insert(debug::PRINT_UINT256.into(), debug::print_uint256);
    hints.insert(debug::PRINT_UINT384.into(), debug::print_uint384);
    hints.insert(utils::HINT_BIT_LENGTH.into(), utils::hint_bit_length);
    hints
}
