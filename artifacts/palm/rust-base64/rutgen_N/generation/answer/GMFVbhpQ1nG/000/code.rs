// Answer 0

#[test]
fn test_valid_alphabet() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let result = base64::new(alphabet);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_length() {
    let alphabet = "A"; // too short
    let result = base64::new(alphabet);
    assert!(result.is_err());
}

#[test]
fn test_unprintable_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234\x006789+/"; // contains unprintable byte
    let result = base64::new(alphabet);
    assert!(result.is_err());
}

#[test]
fn test_reserved_byte() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789=/"; // contains reserved byte '='
    let result = base64::new(alphabet);
    assert!(result.is_err());
}

#[test]
fn test_duplicated_byte() {
    let alphabet = "AAABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; // contains duplicated byte 'A'
    let result = base64::new(alphabet);
    assert!(result.is_err());
}

#[test]
fn test_edge_case_alphabet() {
    let alphabet = "ABCDEFHIJKLMNOPQRSTUVWXYZabcdefghijklmnoqrstuvwxyz0123456789+/";
    let result = base64::new(alphabet);
    assert!(result.is_ok());
}

