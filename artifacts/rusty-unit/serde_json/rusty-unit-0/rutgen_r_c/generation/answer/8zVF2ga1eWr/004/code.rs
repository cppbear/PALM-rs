// Answer 0

#[test]
fn test_is_escape_double_quote() {
    let ch = b'"';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_backslash() {
    let ch = b'\\';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_control_character_false() {
    let ch = b'\x1F'; // Control character, should not match since including_control_characters is false
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

#[test]
fn test_is_escape_control_character_true() {
    let ch = b'\x1F'; // Control character, should match since including_control_characters is true
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_non_control_character() {
    let ch = b'a'; // Non-special character
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

