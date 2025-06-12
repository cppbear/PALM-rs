// Answer 0

#[test]
fn test_read_varu32_single_byte() {
    let data: &[u8] = &[0b0000_0011]; // Represents the value 3
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 3);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0001]; // Represents the value 129 (1 * 128 + 1)
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 129);
    assert_eq!(bytes_read, 2);
}

#[test]
fn test_read_varu32_var_length() {
    let data: &[u8] = &[0b1000_0001, 0b1000_0010, 0b0000_0011]; // Represents the value 13185 (1 * 128 * 128 + 2 * 128 + 3)
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 13185);
    assert_eq!(bytes_read, 3);
}

#[test]
fn test_read_varu32_edge_case() {
    let data: &[u8] = &[0b1111_1111]; // Should continue to read expecting more bytes
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 0);
    assert_eq!(bytes_read, 0);
}

#[test]
fn test_read_varu32_empty() {
    let data: &[u8] = &[]; // Input is empty
    let (value, bytes_read) = read_varu32(data);
    assert_eq!(value, 0);
    assert_eq!(bytes_read, 0);
}

