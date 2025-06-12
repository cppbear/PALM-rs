// Answer 0

#[test]
fn test_read_varu32_empty_input() {
    let data: &[u8] = &[];
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_read_varu32_single_byte_non_varint() {
    let data: &[u8] = &[0b0111_1111]; // This should be the last valid byte for a single byte varint
    let result = read_varu32(data);
    assert_eq!(result, (127, 1));
}

#[test]
fn test_read_varu32_single_byte_varint() {
    let data: &[u8] = &[0b1000_0000]; // This should return (0, 0) since it is not a complete varint
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data: &[u8] = &[0b1000_0001, 0b0000_0001]; // Represents the varint value 129 (0b10000001 is the first byte, next byte adds 1)
    let result = read_varu32(data);
    assert_eq!(result, (129, 2));
}

#[test]
fn test_read_varu32_leading_bytes() {
    let data: &[u8] = &[0b1000_0000, 0b1000_0000]; // The first byte makes it invalid for varint, should return (0, 0)
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_read_varu32_complete_varint() {
    let data: &[u8] = &[0b1010_1101, 0b0000_0011]; // Represents 0b00001011011000000000000000000011 which is 715827883
    let result = read_varu32(data);
    assert_eq!(result, (715827883, 2));
}

#[test]
fn test_read_varu32_over_max_bytes() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111]; // Over max bytes for a complete varint
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

