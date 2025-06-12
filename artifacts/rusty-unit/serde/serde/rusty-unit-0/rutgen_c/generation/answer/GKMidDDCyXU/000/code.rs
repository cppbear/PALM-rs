// Answer 0

#[test]
fn test_from_utf8_lossy_valid_utf8() {
    let bytes: &[u8] = b"Hello, World!";
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, Cow::Owned(String::from("Hello, World!")));
}

#[test]
fn test_from_utf8_lossy_invalid_utf8() {
    let bytes: &[u8] = &[0, 159, 146, 150];
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, Cow::Owned(String::from("�")));
}

#[test]
fn test_from_utf8_lossy_partial_utf8() {
    let bytes: &[u8] = b"Hello, \xF0\x90\x80World!";
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, Cow::Owned(String::from("Hello, �World!")));
}

#[test]
fn test_from_utf8_lossy_empty() {
    let bytes: &[u8] = b"";
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, Cow::Owned(String::from("")));
}

#[test]
fn test_from_utf8_lossy_boundary_valid() {
    let bytes: &[u8] = b"\xC2\xA9"; // ©
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, Cow::Owned(String::from("©")));
}

#[test]
fn test_from_utf8_lossy_boundary_invalid() {
    let bytes: &[u8] = &[0xFF, 0xFF]; 
    let result = from_utf8_lossy(bytes);
    assert_eq!(result, Cow::Owned(String::from("��")));
}

