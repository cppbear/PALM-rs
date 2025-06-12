// Answer 0

#[test]
fn test_read_varu32_single_byte() {
    let data = vec![0b0000_0001]; // Represents the number 1
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data = vec![0b1000_0001, 0b0000_0001]; // Represents the number 129 (1*128 + 1)
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 129);
    assert_eq!(bytes_read, 2);
}

#[test]
fn test_read_varu32_boundary_condition() {
    let data = vec![0b1111_1111, 0b0000_0001]; // Represents the number 255 (127 * 2 + 1)
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 255);
    assert_eq!(bytes_read, 2);
}

#[test]
fn test_read_varu32_max_value() {
    let data = vec![0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0111_1111]; // Represents the number 2^32 - 1
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, u32::MAX);
    assert_eq!(bytes_read, 4);
}

#[test]
fn test_read_varu32_empty_input() {
    let data: Vec<u8> = vec![];
    let (result, bytes_read) = read_varu32(&data);
    assert_eq!(result, 0);
    assert_eq!(bytes_read, 0);
}

