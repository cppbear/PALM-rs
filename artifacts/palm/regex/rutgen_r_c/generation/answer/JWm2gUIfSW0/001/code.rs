// Answer 0

#[test]
fn test_write_varu32_with_lower_boundary() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0b01111111); // Testing with n == 0b01111111
    assert_eq!(data, vec![0b01111111]); // Expecting a single byte output
}

#[test]
fn test_write_varu32_with_upper_boundary() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0b10000000); // Testing with n == 0b10000000
    assert_eq!(data, vec![0b10000000 | 0b10000000, 0]); // Expecting two bytes output
}

#[test]
fn test_write_varu32_with_large_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0b11111111111111111111111111111111); // Testing with a large n
    assert_eq!(data, vec![0b11111111 | 0b10000000, 0b11111111 | 0b10000000, 0b11111111 | 0b10000000, 0b11111111, 0]); 
} 

#[test]
fn test_write_varu32_with_zero() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0); // Testing with n == 0
    assert_eq!(data, vec![0]); // Expecting a single byte output
}

