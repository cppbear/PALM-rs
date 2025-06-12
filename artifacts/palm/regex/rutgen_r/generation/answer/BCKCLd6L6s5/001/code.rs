// Answer 0

#[test]
fn test_read_vari32_with_positive_value() {
    let data: &[u8] = &[0b00000001]; // Encoding representing a positive varint (1)
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, 1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_vari32_with_negative_value() {
    let data: &[u8] = &[0b00000011]; // Encoding representing a negative varint (-1)
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, -1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_vari32_with_two_bytes() {
    let data: &[u8] = &[0b00000010, 0b00000001]; // Encoding for 1 in two bytes
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, 2);
    assert_eq!(bytes_read, 2);
}

#[test]
fn test_read_vari32_with_large_negative_value() {
    let data: &[u8] = &[0b11111111, 0b11111111, 0b11111111, 0b11111111]; // Encoding for the negative largest value
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, -2147483647 - 1); // corresponds to -2147483648
    assert_eq!(bytes_read, 4);
}

#[test]
fn test_read_vari32_with_large_positive_value() {
    let data: &[u8] = &[0b00000010, 0b11111111, 0b11111111, 0b11111111]; // Encoding for a large positive value
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, 2147483647);
    assert_eq!(bytes_read, 4);
}

