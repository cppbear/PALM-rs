// Answer 0

#[test]
fn test_decode_four_hex_digits_zero() {
    let a: u8 = 0; // corresponds to '0'
    let b: u8 = 0; // corresponds to '0'
    let c: u8 = 0; // corresponds to '0'
    let d: u8 = 0; // corresponds to '0'
    
    let result = decode_four_hex_digits(a, b, c, d);
    assert_eq!(result, Some(0)); // codepoint == 0 should return Some(0)
}

#[test]
fn test_decode_four_hex_digits_valid_minimum() {
    let a: u8 = 0; // corresponds to '0'
    let b: u8 = 1; // corresponds to '1'
    let c: u8 = 0; // corresponds to '0'
    let d: u8 = 0; // corresponds to '0'
    
    let result = decode_four_hex_digits(a, b, c, d);
    assert_eq!(result, Some(256)); // codepoint == 256 should return Some(256)
}

