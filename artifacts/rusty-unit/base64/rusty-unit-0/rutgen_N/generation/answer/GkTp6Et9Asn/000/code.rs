// Answer 0

#[test]
fn test_decode_valid_base64() {
    let input = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let result = base64::decode(input).unwrap();
    assert_eq!(result, b"Hello World");
}

#[test]
fn test_decode_invalid_base64() {
    let input = "SGVsbG8gV29ybGQ"; // Invalid base64 (missing padding)
    let result = base64::decode(input);
    assert!(result.is_err());
}

#[test]
fn test_decode_empty_string() {
    let input = "";
    let result = base64::decode(input).unwrap();
    assert_eq!(result, b"");
}

#[test]
fn test_decode_large_input() {
    let input = "U29tZSBsYXJnZSBkYXRhIHdpdGggbGFyZ2UgcnVubmVyIC0tIFN1bWVkIGZvbGxvd2luZyBkYXJ0aW5n";
    let result = base64::decode(input).unwrap();
    assert_eq!(
        result,
        b"Some large data with large runner -- Summed following darting"
    );
}

