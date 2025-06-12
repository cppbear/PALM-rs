// Answer 0

#[test]
fn test_write_vari32_negative_case() {
    let mut data: Vec<u8> = Vec::new();
    let n: i32 = -1; // This should trigger the branch where n < 0

    write_vari32(&mut data, n);
    
    assert_eq!(data.len(), 1); // The output should have a length of 1 byte
    assert_eq!(data[0], 0b1111_1110); // Expected output for -1 encoded in varint
}

#[test]
fn test_write_vari32_negative_large_case() {
    let mut data: Vec<u8> = Vec::new();
    let n: i32 = -128; // Test a larger negative number

    write_vari32(&mut data, n);
    
    assert_eq!(data.len(), 2); // The output should have a length of 2 bytes
    assert_eq!(data[0], 0b1000_0000); // Expected first byte for -128
    assert_eq!(data[1], 0b1111_1111); // Expected second byte for -128
}

#[test]
fn test_write_vari32_negative_boundary_case() {
    let mut data: Vec<u8> = Vec::new();
    let n: i32 = -64; // Test a negative number that is not too large

    write_vari32(&mut data, n);
    
    assert_eq!(data.len(), 2); // The output should have a length of 2 bytes
    assert_eq!(data[0], 0b1100_0000); // Expected first byte for -64
    assert_eq!(data[1], 0b1111_1111); // Expected second byte for -64
}

