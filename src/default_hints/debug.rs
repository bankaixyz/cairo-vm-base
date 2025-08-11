use std::collections::HashMap;

use cairo_vm::{
    hint_processor::builtin_hint_processor::{
        builtin_hint_processor_definition::HintProcessorData,
        hint_utils::{get_address_from_var_name, get_integer_from_var_name},
    },
    types::{exec_scope::ExecutionScopes, relocatable::MaybeRelocatable},
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};

pub const PRINT_FELT_HEX: &str = "print(f\"{hex(ids.value)}\")";
pub const PRINT_FELT: &str = "print(f\"{ids.value}\")";
pub const PRINT_STRING: &str = "print(f\"String: {ids.value}\")";
pub const PRINT_UINT256: &str = "print(f\"{hex(ids.value.high * 2 ** 128 + ids.value.low)}\")";
pub const PRINT_UINT384: &str =
    "print(f\"{hex(ids.value.d3 * 2 ** 144 + ids.value.d2 * 2 ** 96 + ids.value.d1 * 2 ** 48 + ids.value.d0)}\")";

pub fn print_felt_hex(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    hint_data: &HintProcessorData,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let value =
        get_integer_from_var_name("value", vm, &hint_data.ids_data, &hint_data.ap_tracking)?;
    println!("Value: {}", value.to_hex_string());
    Ok(())
}

pub fn print_felt(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    hint_data: &HintProcessorData,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let value =
        get_integer_from_var_name("value", vm, &hint_data.ids_data, &hint_data.ap_tracking)?;
    println!("Value: {}", value);
    Ok(())
}

pub fn print_string(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    hint_data: &HintProcessorData,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let value =
        get_integer_from_var_name("value", vm, &hint_data.ids_data, &hint_data.ap_tracking)?;
    let bytes = value.to_bytes_be();
    let ascii = String::from_utf8_lossy(&bytes);
    println!("String: {}", ascii);
    Ok(())
}

pub fn print_uint256(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    hint_data: &HintProcessorData,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let ptr: MaybeRelocatable =
        get_address_from_var_name("value", vm, &hint_data.ids_data, &hint_data.ap_tracking)?;
    if let MaybeRelocatable::RelocatableValue(ptr) = ptr {
        let low = vm.get_integer((ptr + 0)?)?;
        let high = vm.get_integer((ptr + 1)?)?;
        
        let low_bytes = low.to_bytes_be();
        let high_bytes = high.to_bytes_be();
        
        let low_128 = &low_bytes[low_bytes.len().saturating_sub(16)..];
        let high_128 = &high_bytes[high_bytes.len().saturating_sub(16)..];
        
        let mut bytes = Vec::new();
        bytes.extend_from_slice(high_128);
        bytes.extend_from_slice(low_128);
        println!("Value: 0x{}", hex::encode(bytes));
        return Ok(());
    }
    Err(HintError::UnknownHint(
        hint_data.code.to_string().into_boxed_str(),
    ))
}

pub fn print_uint384(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    hint_data: &HintProcessorData,
    _constants: &HashMap<String, Felt252>,
) -> Result<(), HintError> {
    let ptr: MaybeRelocatable =
        get_address_from_var_name("value", vm, &hint_data.ids_data, &hint_data.ap_tracking)?;
    if let MaybeRelocatable::RelocatableValue(ptr) = ptr {
        let d0 = vm.get_integer((ptr + 0)?)?;
        let d1 = vm.get_integer((ptr + 1)?)?;
        let d2 = vm.get_integer((ptr + 2)?)?;
        let d3 = vm.get_integer((ptr + 3)?)?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&d3.to_bytes_be());
        bytes.extend_from_slice(&d2.to_bytes_be());
        bytes.extend_from_slice(&d1.to_bytes_be());
        bytes.extend_from_slice(&d0.to_bytes_be());
        println!("Value: 0x{}", hex::encode(bytes));
    }
    Ok(())
}
