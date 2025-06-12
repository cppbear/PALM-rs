// Answer 0

#[test]
fn test_read_varu32_single_byte() {
    let data: &[u8] = &[0b0000_0001];
    let (result, bytes_read) = read_varu32(data);
    assert_eq!(result, 1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0001]; // Represents 129
    let (result, bytes_read) = read_varu32(data);
    assert_eq!(result, 129);
    assert_eq!(bytes_read, 2);
}

#[test]
fn test_read_varu32_boundary_case() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111, 0b0000_0011]; // Represents 16383
    let (result, bytes_read) = read_varu32(data);
    assert_eq!(result, 16383);
    assert_eq!(bytes_read, 3);
}

#[test]
fn test_read_varu32_overflow() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0010]; // Represents 2 in varint with too many bytes
    let (result, bytes_read) = read_varu32(data);
    assert_eq!(result, 2);
    assert_eq!(bytes_read, 5);
}

#[test]
fn test_read_varu32_empty_input() {
    let data: &[u8] = &[];
    let (result, bytes_read) = read_varu32(data);
    assert_eq!(result, 0);
    assert_eq!(bytes_read, 0);
}

