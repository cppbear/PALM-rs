// Answer 0

#[test]
fn test_read_vari32_positive_single_byte() {
    let data: &[u8] = &[0b0000_0001]; // Encodes 1 as varint.
    let (result, length) = read_vari32(data);
    assert_eq!(result, 1);
    assert_eq!(length, 1);
}

#[test]
fn test_read_vari32_negative_single_byte() {
    let data: &[u8] = &[0b0000_0000]; // Encodes 0 as varint.
    let (result, length) = read_vari32(data);
    assert_eq!(result, 0);
    assert_eq!(length, 1);
}

#[test]
fn test_read_vari32_positive_multiple_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0001]; // Encodes 129 as varint.
    let (result, length) = read_vari32(data);
    assert_eq!(result, 129);
    assert_eq!(length, 2);
}

#[test]
fn test_read_vari32_negative_multiple_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0000]; // Encodes -2 as varint.
    let (result, length) = read_vari32(data);
    assert_eq!(result, -2);
    assert_eq!(length, 2);
}

#[test]
fn test_read_vari32_zero() {
    let data: &[u8] = &[0]; // Encodes 0 as varint.
    let (result, length) = read_vari32(data);
    assert_eq!(result, 0);
    assert_eq!(length, 1);
}

#[test]
fn test_read_vari32_large_number() {
    let data: &[u8] = &[0b1111111, 0b0000001]; // Encodes 127 + (1 << 7) which is 128.
    let (result, length) = read_vari32(data);
    assert_eq!(result, 128);
    assert_eq!(length, 2);
}

#[test]
fn test_read_vari32_edge_case() {
    let data: &[u8] = &[0b1111111, 0b1111111, 0b0000000]; // Edge case to check encoding.
    let (result, length) = read_vari32(data);
    assert_eq!(result, 16383); // 127 + 127*128
    assert_eq!(length, 3);
}

