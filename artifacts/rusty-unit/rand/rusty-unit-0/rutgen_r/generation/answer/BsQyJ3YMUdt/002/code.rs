// Answer 0

#[test]
fn test_read_u32le_valid_input() {
    let input: &[u8] = &[1, 2, 3, 4]; // 0x04030201 in little-endian
    let result = read_u32le(input);
    assert_eq!(result, 0x04030201);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32le_invalid_length_too_short() {
    let input: &[u8] = &[1, 2, 3]; // Length is 3
    read_u32le(input);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32le_invalid_length_too_long() {
    let input: &[u8] = &[1, 2, 3, 4, 5]; // Length is 5
    read_u32le(input);
}

#[test]
fn test_read_u32le_edge_case_min_value() {
    let input: &[u8] = &[0, 0, 0, 0]; // 0x00000000 in little-endian
    let result = read_u32le(input);
    assert_eq!(result, 0);
}

#[test]
fn test_read_u32le_edge_case_max_value() {
    let input: &[u8] = &[255, 255, 255, 255]; // 0xFFFFFFFF in little-endian
    let result = read_u32le(input);
    assert_eq!(result, 0xFFFFFFFF);
}

