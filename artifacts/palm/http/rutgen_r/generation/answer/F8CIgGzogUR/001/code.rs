// Answer 0

#[test]
fn test_as_str_valid_utf8() {
    struct InlineExtension<'a>(&'a [u8], usize);
    
    let data = b"Hello, world!";
    let inline_ext = InlineExtension(data, data.len());
    let result = inline_ext.as_str();
    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_as_str_empty() {
    struct InlineExtension<'a>(&'a [u8], usize);
    
    let data: &[u8] = b"";
    let inline_ext = InlineExtension(data, 0);
    let result = inline_ext.as_str();
    assert_eq!(result, "");
}

#[test]
#[should_panic]
fn test_as_str_panic_length_too_large() {
    struct InlineExtension<'a>(&'a [u8], usize);
    
    let data = b"Test";
    let inline_ext = InlineExtension(data, 5); // Length is too large
    let _ = inline_ext.as_str();
}

#[test]
#[should_panic]
fn test_as_str_panic_invalid_utf8() {
    struct InlineExtension<'a>(&'a [u8], usize);
    
    let data = &[0, 159, 146, 150]; // invalid UTF-8 sequence
    let inline_ext = InlineExtension(data, data.len());
    let _ = inline_ext.as_str();
}

