// Answer 0

#[test]
fn test_read_varu32_single_byte() {
    let data: &[u8] = &[0b0000_0001]; // Single byte less than 0b1000_0000
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0001]; // Two bytes
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 1 + (1 << 7)); // 1 from first byte + 1 from second byte shifted
    assert_eq!(bytes_read, 2);
}

#[test]
fn test_read_varu32_three_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b1000_0010, 0b0000_0001]; // Three bytes
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 2 + (1 << 14)); // 2 from second byte + 1 from third byte shifted
    assert_eq!(bytes_read, 3);
}

#[test]
fn test_read_varu32_boundary_value() {
    let data: &[u8] = &[0b0111_1111]; // Maximum single byte value
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 127); // Max value for one byte
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_varu32_zero_case() {
    let data: &[u8] = &[0b0000_0000]; // Single byte zero
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 0); // Expecting 0
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_varu32_exceed_limit() {
    let data: &[u8] = &[0b1000_0001, 0b1000_0001, 0b1000_0001, 0b0000_0001]; // Four bytes
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 1 + (1 << 7) + (1 << 14)); // Calculate properly across four bytes
    assert_eq!(bytes_read, 4);
}

