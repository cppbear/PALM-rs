// Answer 0

#[test]
fn test_read_vari32_positive() {
    let data: &[u8] = &[0b00000010]; // Encodes the value 1
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, 1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_vari32_negative() {
    let data: &[u8] = &[0b00000011]; // Encodes the value -1
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, -1);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_vari32_zero() {
    let data: &[u8] = &[0b00000000]; // Encodes the value 0
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, 0);
    assert_eq!(bytes_read, 1);
}

#[test]
fn test_read_vari32_multiple_bytes() {
    let data: &[u8] = &[0b00000100, 0b00000001]; // Represents a large positive number
    let (result, bytes_read) = read_vari32(data);
    assert_eq!(result, 4);
    assert_eq!(bytes_read, 2);
}

#[should_panic]
fn test_read_vari32_invalid_data() {
    let data: &[u8] = &[0b11111111]; // Invalid encoding scenario; may cause confusion.
    read_vari32(data);
}

