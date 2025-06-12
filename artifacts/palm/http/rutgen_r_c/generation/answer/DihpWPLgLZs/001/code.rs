// Answer 0

#[test]
fn test_new_valid_input() {
    let input = b"GET"; // valid HTTP method
    let result = AllocatedExtension::new(input);
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_input() {
    let input = b"\xFF"; // invalid byte that will trigger InvalidMethod
    let result = AllocatedExtension::new(input);
    assert!(result.is_err());
}

#[test]
fn test_new_empty_input() {
    let input: &[u8] = b""; // empty input, should be valid
    let result = AllocatedExtension::new(input);
    assert!(result.is_ok());
}

#[test]
fn test_new_max_length_input() {
    let input = b"OPTIONS"; // valid HTTP method
    let result = AllocatedExtension::new(input);
    assert!(result.is_ok());
}

#[test]
fn test_new_long_invalid_input() {
    let input = b"INVALIDMETHOD\xFF"; // contains an invalid byte
    let result = AllocatedExtension::new(input);
    assert!(result.is_err());
}

