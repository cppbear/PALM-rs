// Answer 0

#[test]
fn test_from_static_valid_string() {
    let byte_str = ByteStr::from_static("hello");
    assert_eq!(byte_str.bytes.as_ref(), b"hello");
}

#[test]
fn test_from_static_empty_string() {
    let byte_str = ByteStr::from_static("");
    assert_eq!(byte_str.bytes.as_ref(), b"");
}

#[test]
fn test_from_static_unicode_string() {
    let byte_str = ByteStr::from_static("こんにちは");
    assert_eq!(byte_str.bytes.as_ref(), "こんにちは".as_bytes());
}

#[test]
fn test_from_static_single_character() {
    let byte_str = ByteStr::from_static("a");
    assert_eq!(byte_str.bytes.as_ref(), b"a");
}

