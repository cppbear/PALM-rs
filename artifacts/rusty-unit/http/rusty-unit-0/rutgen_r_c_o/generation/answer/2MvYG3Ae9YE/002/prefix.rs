// Answer 0

#[test]
fn test_from_lowercase_valid() {
    let input = b"content-length";
    let result = HeaderName::from_lowercase(input);
}

#[test]
#[should_panic]
fn test_from_lowercase_invalid_uppercase() {
    let input = b"Content-Length";
    let result = HeaderName::from_lowercase(input);
}

#[test]
#[should_panic]
fn test_from_lowercase_invalid_byte() {
    let input = b"content-lenght\xFF"; // contains invalid byte
    let result = HeaderName::from_lowercase(input);
}

#[test]
fn test_from_lowercase_empty() {
    let input = b""; // empty input
    let result = HeaderName::from_lowercase(input);
}

#[test]
#[should_panic]
fn test_from_lowercase_too_long() {
    let input = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"; // more than 64 bytes
    let result = HeaderName::from_lowercase(input);
}

#[test]
#[should_panic]
fn test_from_lowercase_invalid_character() {
    let input = b"content-length!\x00"; // contains invalid character
    let result = HeaderName::from_lowercase(input);
}

