// Answer 0

#[test]
fn test_borrow_empty() {
    let header_name = HeaderName::from_bytes(&[]).unwrap();
    header_name.borrow();
}

#[test]
fn test_borrow_single_byte() {
    let header_name = HeaderName::from_bytes(&[b'A']).unwrap();
    header_name.borrow();
}

#[test]
fn test_borrow_max_length() {
    let valid_bytes = (0x21..=0x7E).map(|b| b as u8).collect::<Vec<u8>>();
    let header_name = HeaderName::from_bytes(&valid_bytes).unwrap();
    header_name.borrow();
}

#[test]
fn test_borrow_with_standard_header() {
    let header_name = HeaderName::from_bytes(b"accept").unwrap();
    header_name.borrow();
}

#[test]
fn test_borrow_custom_header() {
    let custom_bytes = b"custom-header-123";
    let header_name = HeaderName::from_bytes(custom_bytes).unwrap();
    header_name.borrow();
}

#[test]
fn test_borrow_invalid_header_name_too_long() {
    let too_long_bytes = (0x21..=0x7E).cycle().take(65).collect::<Vec<u8>>();
    let result = HeaderName::from_bytes(&too_long_bytes);
    assert!(result.is_err());
}

#[test]
fn test_borrow_invalid_header_name_invalid_byte() {
    let invalid_bytes = [b'A', 0x00, b'C'];
    let result = HeaderName::from_bytes(&invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_borrow_varied_valid_characters() {
    let mixed_valid_bytes = b"!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let header_name = HeaderName::from_bytes(mixed_valid_bytes).unwrap();
    header_name.borrow();
}

