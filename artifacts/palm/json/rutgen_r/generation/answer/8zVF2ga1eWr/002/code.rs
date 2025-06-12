// Answer 0

#[test]
fn test_is_escape_with_char_double_quote() {
    let ch = b'"';
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_char_backslash() {
    let ch = b'\\';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_control_character_and_including_true() {
    let ch = 0x1;  // Control character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_control_character_and_including_false() {
    let ch = 0x1;  // Control character
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

#[test]
fn test_is_escape_with_non_control_character() {
    let ch = b'a';  // Non-control character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

