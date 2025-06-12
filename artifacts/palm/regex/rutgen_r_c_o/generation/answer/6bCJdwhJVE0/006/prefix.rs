// Answer 0

#[test]
fn test_escape_unicode_whitespace_single_space() {
    let bytes = [0x20];
    escape_unicode(&bytes);
}

#[test]
fn test_escape_unicode_whitespace_newline_and_carriage_return() {
    let bytes = [0x0A, 0x0D];
    escape_unicode(&bytes);
}

#[test]
fn test_escape_unicode_whitespace_delete_character() {
    let bytes = [0x7F];
    escape_unicode(&bytes);
}

#[test]
fn test_escape_unicode_mixed_characters() {
    let bytes = [0x41, 0x20, 0x0D, 0x20, 0x0A];
    escape_unicode(&bytes);
}

