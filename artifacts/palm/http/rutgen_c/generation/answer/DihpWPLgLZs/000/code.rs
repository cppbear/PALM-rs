// Answer 0

#[test]
fn test_new_valid_input() {
    let valid_input: &[u8] = b"GET";
    let result = AllocatedExtension::new(valid_input);
    assert!(result.is_ok());
    let allocated_extension = result.unwrap();
    assert_eq!(allocated_extension.as_str(), "GET");
}

#[test]
fn test_new_invalid_input() {
    let invalid_input: &[u8] = b"INVALID_METHOD_!";
    let result = AllocatedExtension::new(invalid_input);
    assert!(result.is_err());
}

#[test]
fn test_new_empty_input() {
    let empty_input: &[u8] = b"";
    let result = AllocatedExtension::new(empty_input);
    assert!(result.is_ok());
    let allocated_extension = result.unwrap();
    assert_eq!(allocated_extension.as_str(), "");
}

#[test]
#[should_panic]
fn test_new_null_byte() {
    let null_byte_input: &[u8] = b"GET\0";
    AllocatedExtension::new(null_byte_input).unwrap();
}

