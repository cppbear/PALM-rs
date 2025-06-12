// Answer 0

#[test]
fn test_decode_valid_input() {
    let input = "SGVsbG8gd29ybGQ="; // "Hello world" in base64
    let expected = b"Hello world".to_vec();
    let result = decode(input);
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_decode_empty_input() {
    let input = ""; // Empty base64 input
    let expected = b"".to_vec();
    let result = decode(input);
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[should_panic]
fn test_decode_invalid_character() {
    let input = "SGVsbG8gd29ybGQ#"; // Invalid character in base64
    let _result = decode(input);
}

#[test]
#[should_panic]
fn test_decode_incomplete_padding() {
    let input = "SGVsbG8gd29ybGQ"; // Incomplete padding
    let _result = decode(input);
}

#[test]
fn test_decode_valid_padding() {
    let input = "SGVsbG8="; // "Hello" in base64
    let expected = b"Hello".to_vec();
    let result = decode(input);
    assert_eq!(result.unwrap(), expected);
}

