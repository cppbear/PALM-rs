// Answer 0

#[test]
fn test_from_lowercase_valid() {
    let result = HeaderName::from_lowercase(b"content-length").unwrap();
}

#[test]
fn test_from_lowercase_valid_custom() {
    let result = HeaderName::from_lowercase(b"x-custom-header").unwrap();
}

#[test]
fn test_from_lowercase_empty() {
    let result = HeaderName::from_lowercase(b"");
}

#[test]
fn test_from_lowercase_overflow() {
    let result = HeaderName::from_lowercase(b"a").unwrap();
}

#[test]
fn test_from_lowercase_invalid_character() {
    let result = HeaderName::from_lowercase(b"\xFF");
}

#[test]
fn test_from_lowercase_invalid_high_byte() {
    let result = HeaderName::from_lowercase(b"\x80"); 
}

#[test]
fn test_from_lowercase_uppercase() {
    let result = HeaderName::from_lowercase(b"Content-Length");
}

#[test]
fn test_from_lowercase_mixed_case() {
    let result = HeaderName::from_lowercase(b"Content-Type");
}

#[test]
fn test_from_lowercase_valid_range() {
    let input = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 64 valid lowercase characters
    let result = HeaderName::from_lowercase(input).unwrap();
} 

#[test]
fn test_from_lowercase_invalid_range() {
    let input = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA"; // 63 valid and one invalid
    let result = HeaderName::from_lowercase(input);
} 

#[test]
fn test_from_lowercase_panic_condition_invalid_bytes() {
    let input = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xFF]; // Invalid byte 0xFF present
    let result = HeaderName::from_lowercase(input);
}

