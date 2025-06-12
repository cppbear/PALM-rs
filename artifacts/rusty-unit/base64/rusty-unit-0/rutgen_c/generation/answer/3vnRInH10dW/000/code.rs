// Answer 0

#[test]
fn test_read_u64_valid() {
    let input: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
    let result = read_u64(input);
    assert_eq!(result, 1);
}

#[test]
fn test_read_u64_large_number() {
    let input: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let result = read_u64(input);
    assert_eq!(result, u64::MAX);
}

#[should_panic]
fn test_read_u64_insufficient_length() {
    let input: &[u8] = &[0x00]; // Only 1 byte
    let _result = read_u64(input);
}

#[should_panic]
fn test_read_u64_boundary_exceed() {
    let input: &[u8] = &[0x00; 9]; // 9 bytes, one too many
    let _result = read_u64(input);
}

