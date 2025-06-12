// Answer 0

#[test]
fn test_as_str_valid_utf8() {
    struct InlineExtension<'a>(&'a [u8], usize);

    let valid_utf8_data: &[u8] = b"Hello, world!";
    let len: usize = valid_utf8_data.len();

    let inline_extension = InlineExtension(valid_utf8_data, len);
    let result = inline_extension.as_str();
    
    assert_eq!(result, "Hello, world!");
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    struct InlineExtension<'a>(&'a [u8], usize);

    let invalid_utf8_data: &[u8] = &[0, 159, 146, 150]; // Invalid UTF-8 sequence
    let len: usize = invalid_utf8_data.len();

    let inline_extension = InlineExtension(invalid_utf8_data, len);
    
    // This should panic due to invalid UTF-8
    let _result = inline_extension.as_str();
}

