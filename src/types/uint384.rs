use crate::cairo_type::CairoType;
use crate::types::{hex_bytes_padded, FromAnyStr};
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UInt384(pub BigUint);

impl UInt384 {
    fn to_limbs(&self) -> [Vec<u8>; 4] {
        let bytes = self.0.to_bytes_be();
        let mut padded = [0u8; 48];
        let start = 48 - bytes.len();
        padded[start..].copy_from_slice(&bytes);

        [
            padded[36..48].to_vec(),
            padded[24..36].to_vec(),
            padded[12..24].to_vec(),
            padded[0..12].to_vec(),
        ]
    }
}

impl CairoType for UInt384 {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError> {
        let d0 = BigUint::from_bytes_be(&vm.get_integer((address + 0)?)?.to_bytes_be());
        let d1 = BigUint::from_bytes_be(&vm.get_integer((address + 1)?)?.to_bytes_be());
        let d2 = BigUint::from_bytes_be(&vm.get_integer((address + 2)?)?.to_bytes_be());
        let d3 = BigUint::from_bytes_be(&vm.get_integer((address + 3)?)?.to_bytes_be());
        let bigint = d3 << 288 | d2 << 192 | d1 << 96 | d0;
        Ok(Self(bigint))
    }

    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError> {
        let limbs = self.to_limbs();

        vm.insert_value((address + 0)?, Felt252::from_bytes_be_slice(&limbs[0]))?;
        vm.insert_value((address + 1)?, Felt252::from_bytes_be_slice(&limbs[1]))?;
        vm.insert_value((address + 2)?, Felt252::from_bytes_be_slice(&limbs[2]))?;
        vm.insert_value((address + 3)?, Felt252::from_bytes_be_slice(&limbs[3]))?;

        Ok((address + 4)?)
    }

    fn n_fields() -> usize {
        4
    }
}

impl FromAnyStr for UInt384 {
    fn from_any_str(s: &str) -> Result<Self, String> {
        if !s.starts_with("0x") && !s.starts_with("0X") {
            if let Some(value) = BigUint::parse_bytes(s.as_bytes(), 10) {
                return Ok(UInt384(value));
            }
        }
        // If it has a prefix or decimal parsing fails, treat as hex.
        let bytes = hex_bytes_padded(s, Some(48))?; // 384 bits
        Ok(UInt384(BigUint::from_bytes_be(&bytes)))
    }
}
