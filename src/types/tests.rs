// Tests for automatic deserialize functionality for all Cairo types
// These tests ensure that all type implementations work correctly with:
// - String inputs (hex with/without prefixes, decimal)
// - Numeric inputs (JSON numbers)
// - Edge cases (empty strings, invalid inputs, overflow)
// - Vector deserialization for arrays of values
#[cfg(test)]
mod serde_tests {
    use crate::types::{felt, uint256, uint256_32, uint384};
    use serde::Deserialize;

    // Test structs - now clean without any serde attributes!
    #[derive(Debug, Deserialize, PartialEq)]
    struct FeltWrapper {
        value: felt::Felt,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Uint256Wrapper {
        value: uint256::Uint256,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct UInt384Wrapper {
        value: uint384::UInt384,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Uint256Bits32Wrapper {
        value: uint256_32::Uint256Bits32,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct FeltVecWrapper {
        values: Vec<felt::Felt>,
    }

    mod serialization_tests {
        use super::*;
        use serde_json;

        #[test]
        fn test_felt_serialize_hex() {
            let felt = felt::Felt(cairo_vm::Felt252::from(255));
            let json = serde_json::to_string(&felt).unwrap();
            assert_eq!(
                json,
                "\"0x00000000000000000000000000000000000000000000000000000000000000ff\""
            );
        }

        #[test]
        fn test_uint256_serialize_hex() {
            let uint = uint256::Uint256(num_bigint::BigUint::from(255u32));
            let json = serde_json::to_string(&uint).unwrap();
            assert_eq!(
                json,
                "\"0x00000000000000000000000000000000000000000000000000000000000000ff\""
            );
        }

        #[test]
        fn test_uint384_serialize_hex() {
            let uint = uint384::UInt384(num_bigint::BigUint::from(255u32));
            let json = serde_json::to_string(&uint).unwrap();
            // UInt384 = 48 bytes = 96 hex chars + "0x" prefix
            assert_eq!(json, "\"0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ff\"");
        }

        #[test]
        fn test_uint256_bits32_serialize_hex() {
            let uint = uint256_32::Uint256Bits32(num_bigint::BigUint::from(255u32));
            let json = serde_json::to_string(&uint).unwrap();
            assert_eq!(
                json,
                "\"0x00000000000000000000000000000000000000000000000000000000000000ff\""
            );
        }

        #[test]
        fn test_zero_values_serialize() {
            let felt_zero = felt::Felt(cairo_vm::Felt252::from(0));
            let uint256_zero = uint256::Uint256(num_bigint::BigUint::from(0u32));

            assert_eq!(
                serde_json::to_string(&felt_zero).unwrap(),
                "\"0x0000000000000000000000000000000000000000000000000000000000000000\""
            );
            assert_eq!(
                serde_json::to_string(&uint256_zero).unwrap(),
                "\"0x0000000000000000000000000000000000000000000000000000000000000000\""
            );
        }

        #[test]
        fn test_large_values_serialize() {
            let large_val =
                num_bigint::BigUint::parse_bytes(b"123456789abcdef123456789abcdef", 16).unwrap();
            let uint256 = uint256::Uint256(large_val.clone());
            let uint384 = uint384::UInt384(large_val);

            let json256 = serde_json::to_string(&uint256).unwrap();
            let json384 = serde_json::to_string(&uint384).unwrap();

            // Should be hex strings with 0x prefix
            assert!(json256.starts_with("\"0x"));
            assert!(json384.starts_with("\"0x"));
            assert!(json256.ends_with("\""));
            assert!(json384.ends_with("\""));
        }

        #[test]
        fn test_round_trip_serialization() {
            let original = uint256::Uint256(num_bigint::BigUint::from(12345u32));
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: uint256::Uint256 = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }
    }

    mod felt_tests {
        use super::*;
        use cairo_vm::Felt252;

        #[test]
        fn test_felt_deserialize_from_string_hex() {
            let json = r#"{"value": "0x1a2b3c"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::from_hex("0x1a2b3c").unwrap());
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_deserialize_from_string_hex_uppercase() {
            let json = r#"{"value": "0X1A2B3C"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::from_hex("0x1A2B3C").unwrap());
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_deserialize_from_string_decimal() {
            let json = r#"{"value": "123"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::from(123u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_deserialize_from_number() {
            let json = r#"{"value": 123}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::from(123u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_deserialize_from_large_number() {
            let json = r#"{"value": 18446744073709551615}"#; // u64::MAX
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::from(u64::MAX));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_deserialize_zero() {
            let json = r#"{"value": "0"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::ZERO);
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_deserialize_hex_without_prefix() {
            let json = r#"{"value": "FF"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(Felt252::from(255u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_felt_vec_deserialize() {
            let json = r#"{"values": ["0x1a", "123", "0xFF"]}"#;
            let wrapper: FeltVecWrapper = serde_json::from_str(json).unwrap();
            let expected = vec![
                felt::Felt(Felt252::from(26u64)),
                felt::Felt(Felt252::from(123u64)),
                felt::Felt(Felt252::from(255u64)),
            ];
            assert_eq!(wrapper.values, expected);
        }

        #[test]
        fn test_felt_deserialize_invalid_string() {
            let json = r#"{"value": "invalid_hex"}"#;
            let result: Result<FeltWrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }
    }

    mod uint256_tests {
        use super::*;
        use num_bigint::BigUint;

        #[test]
        fn test_uint256_deserialize_from_string_hex() {
            let json = r#"{"value": "0x1a2b3c4d5e6f"}"#;
            let wrapper: Uint256Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256::Uint256(BigUint::from(0x1a2b3c4d5e6fu64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_deserialize_from_string_decimal() {
            let json = r#"{"value": "123456789"}"#;
            let wrapper: Uint256Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256::Uint256(BigUint::from(123456789u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_deserialize_from_number() {
            let json = r#"{"value": 123456789}"#;
            let wrapper: Uint256Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256::Uint256(BigUint::from(123456789u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_deserialize_large_hex() {
            let json = r#"{"value": "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"}"#;
            let wrapper: Uint256Wrapper = serde_json::from_str(json).unwrap();
            // This is 2^256 - 1
            let max_256 = BigUint::from(2u64).pow(256) - BigUint::from(1u64);
            let expected = uint256::Uint256(max_256);
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_deserialize_zero() {
            let json = r#"{"value": "0"}"#;
            let wrapper: Uint256Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256::Uint256(BigUint::from(0u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_deserialize_overflow() {
            // Test with a value larger than 256 bits
            let json = r#"{"value": "0x10000000000000000000000000000000000000000000000000000000000000000"}"#;
            let result: Result<Uint256Wrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn test_uint256_deserialize_invalid_hex() {
            let json = r#"{"value": "0xGGGG"}"#;
            let result: Result<Uint256Wrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }
    }

    mod uint384_tests {
        use super::*;
        use num_bigint::BigUint;

        #[test]
        fn test_uint384_deserialize_from_string_hex() {
            let json = r#"{"value": "0x1a2b3c4d5e6f"}"#;
            let wrapper: UInt384Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint384::UInt384(BigUint::from(0x1a2b3c4d5e6fu64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint384_deserialize_from_string_decimal() {
            let json = r#"{"value": "123456789012345678901234567890"}"#;
            let wrapper: UInt384Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint384::UInt384(
                BigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap(),
            );
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint384_deserialize_from_number() {
            let json = r#"{"value": 123456789}"#;
            let wrapper: UInt384Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint384::UInt384(BigUint::from(123456789u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint384_deserialize_large_hex() {
            // Test with max 384-bit value
            let json = r#"{"value": "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"}"#;
            let wrapper: UInt384Wrapper = serde_json::from_str(json).unwrap();
            let max_384 = BigUint::from(2u64).pow(384) - BigUint::from(1u64);
            let expected = uint384::UInt384(max_384);
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint384_deserialize_zero() {
            let json = r#"{"value": "0"}"#;
            let wrapper: UInt384Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint384::UInt384(BigUint::from(0u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint384_deserialize_overflow() {
            // Test with a value larger than 384 bits
            let json = r#"{"value": "0x1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"}"#;
            let result: Result<UInt384Wrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn test_uint384_deserialize_invalid_hex() {
            let json = r#"{"value": "0xZZZZ"}"#;
            let result: Result<UInt384Wrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }
    }

    mod uint256_bits32_tests {
        use super::*;
        use num_bigint::BigUint;

        #[test]
        fn test_uint256_bits32_deserialize_from_string_hex() {
            let json = r#"{"value": "0x1a2b3c4d5e6f"}"#;
            let wrapper: Uint256Bits32Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256_32::Uint256Bits32(BigUint::from(0x1a2b3c4d5e6fu64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_bits32_deserialize_from_string_decimal() {
            let json = r#"{"value": "123456789"}"#;
            let wrapper: Uint256Bits32Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256_32::Uint256Bits32(BigUint::from(123456789u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_bits32_deserialize_from_number() {
            let json = r#"{"value": 123456789}"#;
            let wrapper: Uint256Bits32Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256_32::Uint256Bits32(BigUint::from(123456789u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_bits32_deserialize_large_hex() {
            let json = r#"{"value": "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"}"#;
            let wrapper: Uint256Bits32Wrapper = serde_json::from_str(json).unwrap();
            let max_256 = BigUint::from(2u64).pow(256) - BigUint::from(1u64);
            let expected = uint256_32::Uint256Bits32(max_256);
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_bits32_deserialize_zero() {
            let json = r#"{"value": "0"}"#;
            let wrapper: Uint256Bits32Wrapper = serde_json::from_str(json).unwrap();
            let expected = uint256_32::Uint256Bits32(BigUint::from(0u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_uint256_bits32_deserialize_overflow() {
            // Test with a value larger than 256 bits
            let json = r#"{"value": "0x10000000000000000000000000000000000000000000000000000000000000000"}"#;
            let result: Result<Uint256Bits32Wrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn test_uint256_bits32_deserialize_invalid_hex() {
            let json = r#"{"value": "0xYYYY"}"#;
            let result: Result<Uint256Bits32Wrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }
    }

    mod edge_case_tests {
        use super::*;

        #[test]
        fn test_hex_with_underscore_separators() {
            let json = r#"{"value": "0x1a_2b_3c_4d"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(cairo_vm::Felt252::from(0x1a2b3c4du64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_odd_length_hex() {
            let json = r#"{"value": "0x1"}"#;
            let wrapper: FeltWrapper = serde_json::from_str(json).unwrap();
            let expected = felt::Felt(cairo_vm::Felt252::from(1u64));
            assert_eq!(wrapper.value, expected);
        }

        #[test]
        fn test_empty_string_behavior() {
            // Empty string should either fail or return zero - let's document the actual behavior
            let json = r#"{"value": ""}"#;
            let result: Result<FeltWrapper, _> = serde_json::from_str(json);
            match result {
                Ok(wrapper) => {
                    // If it succeeds, it should be zero
                    let expected = felt::Felt(cairo_vm::Felt252::ZERO);
                    assert_eq!(wrapper.value, expected);
                }
                Err(_) => {
                    // This is also acceptable behavior for empty string
                }
            }
        }

        #[test]
        fn test_null_value_fails() {
            let json = r#"{"value": null}"#;
            let result: Result<FeltWrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn test_boolean_fails() {
            let json = r#"{"value": true}"#;
            let result: Result<FeltWrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn test_array_fails() {
            let json = r#"{"value": [1, 2, 3]}"#;
            let result: Result<FeltWrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }

        #[test]
        fn test_object_fails() {
            let json = r#"{"value": {"nested": "object"}}"#;
            let result: Result<FeltWrapper, _> = serde_json::from_str(json);
            assert!(result.is_err());
        }
    }

    mod comprehensive_format_tests {
        use super::*;
        use cairo_vm::Felt252;

        #[test]
        fn test_all_types_with_same_value() {
            let value_str = "12345";
            let value_num = 12345u64;

            // Test Felt
            let felt_json = format!(r#"{{"value": "{value_str}"}}"#);
            let felt_wrapper: FeltWrapper = serde_json::from_str(&felt_json).unwrap();
            assert_eq!(felt_wrapper.value, felt::Felt(Felt252::from(value_num)));

            let felt_json_num = format!(r#"{{"value": {value_num}}}"#);
            let felt_wrapper_num: FeltWrapper = serde_json::from_str(&felt_json_num).unwrap();
            assert_eq!(felt_wrapper_num.value, felt::Felt(Felt252::from(value_num)));

            // Test Uint256
            let uint256_json = format!(r#"{{"value": "{value_str}"}}"#);
            let uint256_wrapper: Uint256Wrapper = serde_json::from_str(&uint256_json).unwrap();
            assert_eq!(
                uint256_wrapper.value,
                uint256::Uint256(num_bigint::BigUint::from(value_num))
            );

            let uint256_json_num = format!(r#"{{"value": {value_num}}}"#);
            let uint256_wrapper_num: Uint256Wrapper =
                serde_json::from_str(&uint256_json_num).unwrap();
            assert_eq!(
                uint256_wrapper_num.value,
                uint256::Uint256(num_bigint::BigUint::from(value_num))
            );

            // Test UInt384
            let uint384_json = format!(r#"{{"value": "{value_str}"}}"#);
            let uint384_wrapper: UInt384Wrapper = serde_json::from_str(&uint384_json).unwrap();
            assert_eq!(
                uint384_wrapper.value,
                uint384::UInt384(num_bigint::BigUint::from(value_num))
            );

            let uint384_json_num = format!(r#"{{"value": {value_num}}}"#);
            let uint384_wrapper_num: UInt384Wrapper =
                serde_json::from_str(&uint384_json_num).unwrap();
            assert_eq!(
                uint384_wrapper_num.value,
                uint384::UInt384(num_bigint::BigUint::from(value_num))
            );

            // Test Uint256Bits32
            let uint256_32_json = format!(r#"{{"value": "{value_str}"}}"#);
            let uint256_32_wrapper: Uint256Bits32Wrapper =
                serde_json::from_str(&uint256_32_json).unwrap();
            assert_eq!(
                uint256_32_wrapper.value,
                uint256_32::Uint256Bits32(num_bigint::BigUint::from(value_num))
            );

            let uint256_32_json_num = format!(r#"{{"value": {value_num}}}"#);
            let uint256_32_wrapper_num: Uint256Bits32Wrapper =
                serde_json::from_str(&uint256_32_json_num).unwrap();
            assert_eq!(
                uint256_32_wrapper_num.value,
                uint256_32::Uint256Bits32(num_bigint::BigUint::from(value_num))
            );
        }
    }
}
