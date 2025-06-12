// Answer 0

#[test]
fn test_read_u32le_valid_input() {
    let input: [u8; 4] = [1, 2, 3, 4];
    let result = read_u32le(&input);
    assert_eq!(result, 0x04030201);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32le_invalid_length() {
    let input: [u8; 3] = [1, 2, 3];
    let _result = read_u32le(&input);
}

#[test]
fn test_read_u32le_zero_input() {
    let input: [u8; 4] = [0, 0, 0, 0];
    let result = read_u32le(&input);
    assert_eq!(result, 0);
}

#[test]
fn test_read_u32le_max_input() {
    let input: [u8; 4] = [255, 255, 255, 255];
    let result = read_u32le(&input);
    assert_eq!(result, 0xFFFFFFFF);
}

