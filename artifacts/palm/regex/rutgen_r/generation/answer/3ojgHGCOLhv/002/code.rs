// Answer 0

#[test]
fn test_read_varu32_valid_case() {
    let data = [0b0000_0001, 0b0000_0010]; // Expected to read the varint correctly
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 0b0000_0001 | (0b0000_0010 << 7)); // 0b0000_0001 + (0b0000_0010 << 7) = 128 + 1 = 129
    assert_eq!(bytes_read, 2); // Should read 2 bytes
}

#[test]
fn test_read_varu32_with_full_byte() {
    let data = [0b1000_0000]; // The first byte is set to zero
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 0); // Should return 0 since the first byte indicates continuation
    assert_eq!(bytes_read, 1); // Should read 1 byte
}

#[test]
fn test_read_varu32_empty_data() {
    let data: &[u8] = &[]; // An empty input to check the function's response
    let (result, bytes_read) = read_varu32(data);
    assert_eq!(result, 0); // Should return 0
    assert_eq!(bytes_read, 0); // Should indicate no bytes read
}

#[test]
fn test_read_varu32_single_continuation() {
    let data = [0b1000_0001]; // First byte continues; should result in the original value
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 1); // Should read a value of 1 after processing
    assert_eq!(bytes_read, 1); // Should read 1 byte
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data = [0b1000_0001, 0b0100_0001]; // Should process two bytes correctly
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 1 | (1 << 7)); // Result should represent the value encoded
    assert_eq!(bytes_read, 2); // Should read 2 bytes
}

#[test]
#[should_panic]
fn test_read_varu32_panic_condition() {
    let data = [0b1000_0000]; // ...

    // Trigger panic condition
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 0); // This result should not apply; expecting a panic instead
}

