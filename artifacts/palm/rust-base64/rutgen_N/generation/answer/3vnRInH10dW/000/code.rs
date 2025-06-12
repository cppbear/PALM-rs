// Answer 0

#[test]
fn test_read_u64_valid_input() {
    let input: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
    let result = read_u64(input);
    assert_eq!(result, 1);
}

#[test]
fn test_read_u64_valid_input_large_value() {
    let input: &[u8] = &[0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let result = read_u64(input);
    assert_eq!(result, u64::MAX);
}

#[test]
#[should_panic]
fn test_read_u64_too_short_input() {
    let input: &[u8] = &[0x00, 0x00, 0x00, 0x00]; // Only 4 bytes
    let _result = read_u64(input);
}

#[test]
#[should_panic]
fn test_read_u64_empty_input() {
    let input: &[u8] = &[]; // No bytes
    let _result = read_u64(input);
}

