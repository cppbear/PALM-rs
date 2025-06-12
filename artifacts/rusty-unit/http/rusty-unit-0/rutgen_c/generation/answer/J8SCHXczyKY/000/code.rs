// Answer 0

#[test]
fn test_extension_inline_valid() {
    let src: &[u8] = b"example_extension";
    let result = Method::extension_inline(src);
    assert!(result.is_ok());
    if let Ok(method) = result {
        match method.0 {
            Inner::ExtensionInline(ref inline) => {
                assert_eq!(inline.as_str(), "example_extension");
            },
            _ => panic!("Expected Inner::ExtensionInline"),
        }
    }
}

#[test]
fn test_extension_inline_invalid() {
    let src: &[u8] = b"this_is_a_very_long_extension_string"; // Exceeds MAX length
    let result = Method::extension_inline(src);
    assert!(result.is_err());
}

