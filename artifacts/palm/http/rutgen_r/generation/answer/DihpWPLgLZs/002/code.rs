// Answer 0

#[test]
fn test_new_with_valid_utf8() {
    let valid_input: &[u8] = b"Hello, world!";
    let result = new(valid_input);
    assert!(result.is_ok());

    if let Ok(allocated_extension) = result {
        assert_eq!(allocated_extension.0.len(), valid_input.len());
        assert_eq!(allocated_extension.0.as_ref(), valid_input);
    }
}

#[test]
fn test_new_with_empty_input() {
    let empty_input: &[u8] = b"";
    let result = new(empty_input);
    assert!(result.is_ok());

    if let Ok(allocated_extension) = result {
        assert_eq!(allocated_extension.0.len(), empty_input.len());
    }
}

#[test]
#[should_panic]
fn test_new_with_invalid_utf8() {
    let invalid_input: &[u8] = &[0, 159, 146, 150]; // Invalid UTF-8 sequence
    let _ = new(invalid_input);
}

#[test]
fn test_new_with_non_ascii() {
    let non_ascii_input: &[u8] = "こんにちは".as_bytes(); // Valid UTF-8 but non-ASCII
    let result = new(non_ascii_input);
    assert!(result.is_ok());

    if let Ok(allocated_extension) = result {
        assert_eq!(allocated_extension.0.len(), non_ascii_input.len());
        assert_eq!(allocated_extension.0.as_ref(), non_ascii_input);
    }
}

