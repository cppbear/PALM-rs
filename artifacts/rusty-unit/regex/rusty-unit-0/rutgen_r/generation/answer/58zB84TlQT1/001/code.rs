// Answer 0

#[test]
fn test_new_valid_input() {
    let text: &[u8] = b"Hello, world!";
    let only_utf8 = true;
    
    let result = new(text, only_utf8);
    
    assert_eq!(result.text, text);
    assert_eq!(result.only_utf8, only_utf8);
}

#[test]
fn test_new_valid_input_non_utf8() {
    let text: &[u8] = &[0xFF, 0xFE, 0xFD]; // Non-UTF8 byte sequence
    let only_utf8 = false;

    let result = new(text, only_utf8);
    
    assert_eq!(result.text, text);
    assert_eq!(result.only_utf8, only_utf8);
}

#[should_panic]
fn test_new_empty_text() {
    let text: &[u8] = &[];
    let only_utf8 = true;

    let result = new(text, only_utf8); // This should not panic, but no check for empty text in the context.
}

#[test]
fn test_new_utf8_only() {
    let text: &[u8] = b"Valid UTF8 String";
    let only_utf8 = true;

    let result = new(text, only_utf8);
    
    assert_eq!(result.text, text);
    assert_eq!(result.only_utf8, only_utf8);
}

#[test]
fn test_new_mixed_content() {
    let text: &[u8] = b"Hello, \xFFworld!";
    let only_utf8 = false;

    let result = new(text, only_utf8);
    
    assert_eq!(result.text, text);
    assert_eq!(result.only_utf8, only_utf8);
}

