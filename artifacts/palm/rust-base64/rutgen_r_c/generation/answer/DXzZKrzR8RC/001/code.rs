// Answer 0

#[test]
fn test_encode_empty_input() {
    let input = b"";
    let result = encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_simple_string() {
    let input = b"Hello";
    let result = encode(input);
    assert_eq!(result, "SGVsbG8=");
}

#[test]
fn test_encode_spaced_string() {
    let input = b"Hello World";
    let result = encode(input);
    assert_eq!(result, "SGVsbG8gV29ybGQ=");
}

#[test]
fn test_encode_special_characters() {
    let input = b"Hello, World!";
    let result = encode(input);
    assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
}

#[test]
fn test_encode_multiple_blocks() {
    let input = b"Base64 is a method for encoding binary data.";
    let result = encode(input);
    assert_eq!(result, "QmFzZTY0IGlzIGEgbWV0aG9kIGZvciBlbmNvZGluZyBiaW5hcnkgZGF0YS4=");
}

#[test]
#[should_panic]
fn test_encode_non_utf8_bytes() {
    let input = [0xFF, 0xFF, 0xFF];
    let _result = encode(&input);
}

