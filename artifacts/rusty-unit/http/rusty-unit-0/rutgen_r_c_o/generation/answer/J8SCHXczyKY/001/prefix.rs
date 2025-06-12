// Answer 0

#[test]
fn test_extension_inline_empty() {
    let src: &[u8] = &[];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_max_length_invalid_utf8() {
    let src: &[u8] = &[0xFF; InlineExtension::MAX];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_partial_with_invalid_utf8() {
    let src: &[u8] = &[0xFF, 0xFE, 0xFD, 0xFC, 0xFB];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_contains_valid_utf8() {
    let src: &[u8] = b"valid";
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_over_max_length() {
    let src: &[u8] = &[0xFF; InlineExtension::MAX + 1];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_exact_max_length_invalid_utf8() {
    let src: &[u8] = &[0x80; InlineExtension::MAX];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_non_utf8_length_zero() {
    let src: &[u8] = &[];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_some_invalid_utf8_short() {
    let src: &[u8] = &[0xFF, 0xFE];
    let _ = Method::extension_inline(src);
}

#[test]
fn test_extension_inline_valid_utf8_max_length() {
    let src: &[u8] = b"validutf8maxlen";
    let _ = Method::extension_inline(src);
}

