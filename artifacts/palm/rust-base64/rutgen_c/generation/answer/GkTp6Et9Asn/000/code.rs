// Answer 0

#[test]
fn test_decode_valid_base64() {
    let input = "SGVsbG8gV29ybGQ="; // "Hello World"
    let expected_output = b"Hello World".to_vec();
    let result = decode(input);
    assert_eq!(result.unwrap(), expected_output);
}

#[test]
#[should_panic]
fn test_decode_invalid_base64_invalid_byte() {
    let input = "SGVsbG8gV29ybGQ@"; // Invalid byte '@'
    let result = decode(input);
    result.unwrap(); // This should panic
}

#[test]
#[should_panic]
fn test_decode_invalid_base64_invalid_length() {
    let input = "SGVsbG8gV29ybGQ"; // Invalid length (missing padding)
    let result = decode(input);
    result.unwrap(); // This should panic
}

#[test]
fn test_decode_valid_base64_with_padding() {
    let input = "U28gbG9uZyBhbmQgdGhlbiBhc3BlY3RzIQ=="; // "So long and then aspects!"
    let expected_output = b"So long and then aspects!".to_vec();
    let result = decode(input);
    assert_eq!(result.unwrap(), expected_output);
}

#[test]
fn test_decode_empty_input() {
    let input = "";
    let expected_output = Vec::new();
    let result = decode(input);
    assert_eq!(result.unwrap(), expected_output);
}

