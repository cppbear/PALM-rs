// Answer 0

#[test]
fn test_decode_hex_val_slow_valid_numerals() {
    let inputs: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    for &val in &inputs {
        decode_hex_val_slow(val);
    }
}

#[test]
fn test_decode_hex_val_slow_invalid_uppercase() {
    let invalid_inputs: [u8; 3] = [b'A', b'B', b'C'];
    for &val in &invalid_inputs {
        decode_hex_val_slow(val);
    }
}

#[test]
fn test_decode_hex_val_slow_invalid_lowercase() {
    let invalid_inputs: [u8; 3] = [b'a', b'b', b'c'];
    for &val in &invalid_inputs {
        decode_hex_val_slow(val);
    }
}

#[test]
fn test_decode_hex_val_slow_invalid_non_hex() {
    let invalid_inputs: [u8; 5] = [b'!', b'/', b':', b';', b'@'];
    for &val in &invalid_inputs {
        decode_hex_val_slow(val);
    }
}

