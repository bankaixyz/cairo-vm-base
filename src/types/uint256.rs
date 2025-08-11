use crate::cairo_type::CairoType;
use cairo_vm::{
    types::relocatable::Relocatable,
    vm::{errors::hint_errors::HintError, vm_core::VirtualMachine},
    Felt252,
};
use num_bigint::BigUint;
use serde::Deserialize;
use crate::types::{FromHexStr, hex_bytes_padded};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Uint256(pub BigUint);

impl Uint256 {
    pub fn to_limbs(&self) -> [Felt252; 2] {
        const LIMB_SIZE: u32 = 128;
        let limb_mask = (BigUint::from(1u128) << LIMB_SIZE) - BigUint::from(1u128);

        let lower_limb = &self.0 & &limb_mask;
        let upper_limb = &self.0 >> LIMB_SIZE;

        [
            Felt252::from_bytes_be_slice(&lower_limb.to_bytes_be()),
            Felt252::from_bytes_be_slice(&upper_limb.to_bytes_be()),
        ]
    }
}

impl CairoType for Uint256 {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError> {
        let d0 = BigUint::from_bytes_be(&vm.get_integer((address + 0)?)?.to_bytes_be());
        let d1 = BigUint::from_bytes_be(&vm.get_integer((address + 1)?)?.to_bytes_be());
        let bigint = d1 << 128 | d0;
        Ok(Self(bigint))
    }

    fn to_memory(
        &self,
        vm: &mut VirtualMachine,
        address: Relocatable,
    ) -> Result<Relocatable, HintError> {
        let limbs = self.to_limbs();
        vm.insert_value((address + 0)?, limbs[0])?;
        vm.insert_value((address + 1)?, limbs[1])?;
        Ok((address + 2)?)
    }

    fn n_fields() -> usize {
        2
    }
}

impl FromHexStr for Uint256 {
    fn from_hex_str(s: &str) -> Result<Self, String> {
        let bytes = hex_bytes_padded(s, Some(32))?; // 256 bits
        Ok(Uint256(BigUint::from_bytes_be(&bytes)))
    }
}
