# Cairo VM Base

A foundational Rust library providing common functionality for Cairo Zero projects using the [`cairo-vm`](https://github.com/lambdaclass/cairo-vm) runtime.

## Overview

This library serves as a base dependency for Cairo Zero projects, providing:

- **Unified Cairo VM interface** - Re-exports `cairo-vm` to ensure version consistency across projects
- **Default hint implementations** - Common hints for debugging, cryptography, and utility operations
- **Type system** - `CairoType` trait and implementations for standard types
- **Utilities** - Helper functions for serialization and memory operations

## Features

### Cairo VM Re-export

```rust
use rust_vm_hints::cairo_vm;
```

The library re-exports the cairo-vm crate with features enabled for extensive hints, Cairo 1 support, and modular builtins.

### Type System

The library provides a comprehensive type system with two core traits:

#### BaseCairoType Trait

The `BaseCairoType` trait defines the fundamental operations for Cairo types:

```rust
pub trait BaseCairoType {
    fn from_bytes_be(bytes: &[u8]) -> Self;
    fn bytes_len() -> usize;
}
```

#### CairoType Trait

The `CairoType` trait extends `BaseCairoType` with VM memory operations:

```rust
pub trait CairoType: Sized + FromAnyStr + BaseCairoType {
    fn from_memory(vm: &VirtualMachine, address: Relocatable) -> Result<Self, HintError>;
    fn to_memory(&self, vm: &mut VirtualMachine, address: Relocatable) -> Result<Relocatable, HintError>;
    fn n_fields() -> usize;
}
```

#### Implemented Types

- **`Felt`** - Cairo field element wrapper (32 bytes)
- **`Uint256`** - 256-bit unsigned integer with limb-based memory layout (32 bytes)
- **`UInt384`** - 384-bit unsigned integer for cryptographic operations (48 bytes) 
- **`Uint256Bits32`** - 256-bit unsigned integer with 32-bit limbs (32 bytes)

All types include:
- **Byte length validation** - `from_bytes_be()` validates input length matches expected size
- **Flexible string parsing** - Support hex (`0x` prefix) and decimal formats
- **Automatic serde integration** - Clean serialization/deserialization without attributes

### Default Hints

The library provides a comprehensive set of built-in hints accessible via `default_hint_mapping()`:

#### Debug Hints
- `print_felt` - Print field element values
- `print_felt_hex` - Print field elements in hexadecimal
- `print_string` - Print field elements as ASCII strings
- `print_uint256` / `print_uint384` - Print large integers

#### Cryptographic Hints
- `sha256_finalize` - SHA-256 hash finalization

#### Utility Hints
- `hint_bit_length` - Calculate bit length of values

### Automatic Serde Integration

All Cairo types (`Felt`, `Uint256`, `UInt384`, `Uint256Bits32`) automatically support flexible JSON serialization and deserialization without any attributes needed:

```rust
use serde::{Deserialize, Serialize};
use rust_vm_hints::types::{felt::Felt, uint256::Uint256};

#[derive(Deserialize, Serialize)]
struct BeaconHeaderCairo {
    pub slot: Uint256,           // ✨ No serde attributes needed!
    pub proposer_index: Uint256,
    pub parent_root: Uint256,
    pub state_root: Uint256,
    pub body_root: Uint256,
}
```

#### Flexible Deserialization

All types automatically accept multiple input formats:
- **JSON Strings**: `"0x1a2b3c"`, `"123"`, `"0xFF"`
- **JSON Numbers**: `123`, `0xFF`
- **Mixed Arrays**: `["0x1a2b3c", 123, "999"]`

#### Full-Padding Serialization

All types serialize to fully-padded hex strings for consistency:
- **`Felt`** → `"0x00000000000000000000000000000000000000000000000000000000000000ff"`
- **`Uint256`** → `"0x00000000000000000000000000000000000000000000000000000000000000ff"`
- **`UInt384`** → `"0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ff"`

#### Example JSON

```json
{
  "slot": "0x1a2b3c",         ✓ Hex string input
  "proposer_index": 123,      ✓ JSON number input  
  "parent_root": "999",       ✓ Decimal string input
  "values": [
    "0x1a2b3c",              ✓ Arrays work automatically
    123,
    "999"
  ]
}
```

#### Round-trip Compatibility

Serialization and deserialization are fully compatible:
```rust
let original = Uint256::from_any_str("255")?;
let json = serde_json::to_string(&original)?;  // "0x00...ff"
let restored: Uint256 = serde_json::from_str(&json)?;
assert_eq!(original, restored);
```

### Other Utilities

- **String parsing** - Flexible parsing from hex or decimal strings via `FromAnyStr` trait
- **File operations** - Bincode-compatible file writers for efficient serialization
- **Hex utilities** - Padded hex byte conversion with `hex_bytes_padded`

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-vm-hints = { path = "path/to/cairo-vm-base" }
```

### Basic Example

```rust
use rust_vm_hints::{
    cairo_vm::{VirtualMachine, Felt252},
    cairo_type::CairoType,
    types::felt::Felt,
    default_hints::default_hint_mapping,
};

// Use the CairoType trait
let felt_value = Felt::from_any_str("42")?;
felt_value.to_memory(&mut vm, address)?;

// Use default hints
let hints = default_hint_mapping();
```

### As a Dependency

This library is designed to be used as a dependency in larger Cairo projects, providing consistent VM interfaces and common functionality across your Cairo ecosystem.

## Dependencies

- `cairo-vm` v2.0.1 - Core Cairo Virtual Machine
- `num-bigint` / `num-traits` - Large integer arithmetic  
- `serde` - JSON serialization/deserialization
- `hex` / `bincode` - Data encoding utilities

## License

[Add your license information here]
