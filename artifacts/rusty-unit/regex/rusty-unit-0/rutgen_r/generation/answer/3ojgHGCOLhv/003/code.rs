// Answer 0

#[test]
fn test_read_varu32_empty_input() {
    let data: &[u8] = &[];
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_read_varu32_single_continue_bit() {
    let data: &[u8] = &[0b1000_0000];
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_read_varu32_multiple_continue_bits() {
    let data: &[u8] = &[0b1000_0000, 0b1000_0000];
    let result = read_varu32(data);
    assert_eq!(result, (0, 0));
}

#[test]
fn test_read_varu32_no_continue_bit() {
    let data: &[u8] = &[0b1111_1111];
    let result = read_varu32(data);
    assert_eq!(result, (0b1111_1111, 1));
}

#[test]
fn test_read_varu32_with_varint() {
    let data: &[u8] = &[0b1010_1010, 0b1000_0001];
    let result = read_varu32(data);
    assert_eq!(result, (0b0000_0001_0101_010, 2));
}

