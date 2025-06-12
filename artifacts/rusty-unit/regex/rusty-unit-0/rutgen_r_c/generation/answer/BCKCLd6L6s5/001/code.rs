// Answer 0

#[test]
fn test_read_vari32_positive_varint() {
    let data: &[u8] = &[0b0000_0001]; // Represents a positive varint with the value of 1.
    let result = read_vari32(data);
    assert_eq!(result, (1, 1)); // (n, i) => (1, number of bytes read)
}

#[test]
fn test_read_vari32_negative_varint() {
    let data: &[u8] = &[0b0000_0011]; // Represents a negative varint due to un & 1 != 0 condition.
    let result = read_vari32(data);
    assert_eq!(result, (-2, 1)); // (n, i) => (-2, number of bytes read)
}

#[test]
fn test_read_vari32_large_varint() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0001]; // Represents a large varint that results in (1 << 7) + 1.
    let result = read_vari32(data);
    assert_eq!(result, (128 + 1, 2)); // (n, i) => (129, number of bytes read)
}

#[test]
fn test_read_vari32_zero_varint() {
    let data: &[u8] = &[0b0000_0000]; // Represents zero.
    let result = read_vari32(data);
    assert_eq!(result, (0, 1)); // (n, i) => (0, number of bytes read)
}

#[test]
fn test_read_vari32_edge_case_negative_varint() {
    let data: &[u8] = &[0b0000_0010]; // Represents a negative varint with the value of -1.
    let result = read_vari32(data);
    assert_eq!(result, (-1, 1)); // (n, i) => (-1, number of bytes read)
}

