// Answer 0

#[test]
fn test_new_empty_text_only_utf8() {
    let text: &[u8] = &[];
    let only_utf8 = true;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_empty_text_not_only_utf8() {
    let text: &[u8] = &[];
    let only_utf8 = false;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_ascii_text_only_utf8() {
    let text: &[u8] = b"hello";
    let only_utf8 = true;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_ascii_text_not_only_utf8() {
    let text: &[u8] = b"hello";
    let only_utf8 = false;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_extended_utf8_text_only_utf8() {
    let text: &[u8] = &[0xF0, 0x9F, 0x98, 0x80]; // U+1F600 (😀)
    let only_utf8 = true;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_invalid_utf8_text_not_only_utf8() {
    let text: &[u8] = &[0xFF, 0xFE]; // Invalid UTF-8 sequence
    let only_utf8 = false;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_large_text_only_utf8() {
    let text: &[u8] = b"This is a longer piece of text that should be handled properly.";
    let only_utf8 = true;
    let result = new(text, only_utf8);
}

#[test]
fn test_new_large_text_not_only_utf8() {
    let text: &[u8] = b"This is a longer piece of text that should be handled properly.";
    let only_utf8 = false;
    let result = new(text, only_utf8);
}

