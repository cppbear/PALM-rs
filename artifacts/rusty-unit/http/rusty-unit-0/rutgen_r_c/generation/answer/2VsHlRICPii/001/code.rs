// Answer 0

#[test]
fn test_from_static_valid_utf8() {
    let valid_str = "Hello, world!";
    let byte_str = ByteStr::from_static(valid_str);
    
    assert_eq!(byte_str.bytes.len(), valid_str.len());
    assert_eq!(byte_str.bytes, Bytes::from_static(valid_str.as_bytes()));
}

#[test]
fn test_from_static_empty_string() {
    let empty_str = "";
    let byte_str = ByteStr::from_static(empty_str);
    
    assert_eq!(byte_str.bytes.len(), 0);
    assert_eq!(byte_str.bytes, Bytes::from_static(empty_str.as_bytes()));
}

#[test]
fn test_from_static_multibyte_characters() {
    let multibyte_str = "こんにちは"; // "Hello" in Japanese
    let byte_str = ByteStr::from_static(multibyte_str);
    
    assert_eq!(byte_str.bytes.len(), multibyte_str.len());
    assert_eq!(byte_str.bytes, Bytes::from_static(multibyte_str.as_bytes()));
}

// Testing some valid ASCII with special characters
#[test]
fn test_from_static_special_characters() {
    let special_str = "Café"; // Contains an accented character
    let byte_str = ByteStr::from_static(special_str);
    
    assert_eq!(byte_str.bytes.len(), special_str.len());
    assert_eq!(byte_str.bytes, Bytes::from_static(special_str.as_bytes()));
}

