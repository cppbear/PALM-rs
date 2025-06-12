// Answer 0

#[test]
fn test_is_escape_double_quote() {
    let ch: u8 = b'"';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_backslash() {
    let ch: u8 = b'\\';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_control_character() {
    let ch: u8 = 0x1F; // Control character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_non_escape_character_with_control_true() {
    let ch: u8 = b'a'; // Non-escape character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

#[test]
fn test_is_escape_non_escape_character_with_control_false() {
    let ch: u8 = b'a'; // Non-escape character
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

#[test]
fn test_is_escape_character_below_32_with_control_true() {
    let ch: u8 = 0x1F; // Control character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_character_below_32_with_control_false() {
    let ch: u8 = 0x1F; // Control character
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

