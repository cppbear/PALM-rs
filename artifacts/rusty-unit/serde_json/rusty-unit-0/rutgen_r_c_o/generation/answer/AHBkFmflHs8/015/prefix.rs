// Answer 0

#[test]
fn test_decode_hex_val_slow_out_of_range_lower() {
    let val: u8 = 0;
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_out_of_range_upper() {
    let val: u8 = 255;
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_lower_alpha() {
    let val: u8 = 64; // just below 'A'
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_upper_alpha() {
    let val: u8 = 71; // just above 'F'
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_lower_hex() {
    let val: u8 = 96; // just below 'a'
    let result = decode_hex_val_slow(val);
}

#[test]
fn test_decode_hex_val_slow_invalid_upper_hex() {
    let val: u8 = 103; // just above 'f'
    let result = decode_hex_val_slow(val);
}

