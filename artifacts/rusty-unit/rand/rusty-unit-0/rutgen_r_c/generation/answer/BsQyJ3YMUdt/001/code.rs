// Answer 0

#[test]
fn test_read_u32le_valid_input() {
    let input: [u8; 4] = [1, 2, 3, 4];
    let result = read_u32le(&input);
    assert_eq!(result, 0x04030201);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32le_invalid_length_too_short() {
    let input: [u8; 3] = [1, 2, 3];
    let _result = read_u32le(&input);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_read_u32le_invalid_length_too_long() {
    let input: [u8; 5] = [1, 2, 3, 4, 5];
    let _result = read_u32le(&input);
}

