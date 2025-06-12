// Answer 0

#[test]
fn test_new_valid_input_max_length() {
    let src: &[u8] = b"GET POST";
    let _ = InlineExtension::new(src);
}

#[test]
fn test_new_valid_input_zero_length() {
    let src: &[u8] = b"";
    let _ = InlineExtension::new(src);
}

#[test]
fn test_new_valid_input_partial_valid_length() {
    let src: &[u8] = b"GET";
    let _ = InlineExtension::new(src);
}

#[test]
#[should_panic]
fn test_new_invalid_input_non_method_characters() {
    let src: &[u8] = b"\xFF\xFE";
    let _ = InlineExtension::new(src);
}

#[test]
#[should_panic]
fn test_new_invalid_input_length_exceeds_max() {
    let src: &[u8] = b"GET HTTP METHODS FOR TESTING";
    let _ = InlineExtension::new(src);
}

#[test]
#[should_panic]
fn test_new_invalid_input_first_invalid_character() {
    let src: &[u8] = b"\x00INVALID";
    let _ = InlineExtension::new(src);
}

#[test]
fn test_new_valid_input_with_special_characters() {
    let src: &[u8] = b"GET&POST";
    let _ = InlineExtension::new(src);
}

