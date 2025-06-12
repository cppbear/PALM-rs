// Answer 0

#[test]
fn test_is_escape_with_double_quote() {
    let ch = b'"';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_backslash() {
    let ch = b'\\';
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_control_character_including() {
    let ch = 0x1F; // Control character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_control_character_excluding() {
    let ch = 0x1F; // Control character
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

#[test]
fn test_is_escape_with_non_control_character() {
    let ch = b'a'; // Non-control character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

#[test]
fn test_is_escape_with_zero_control_character_including() {
    let ch = 0x00; // Null character
    let including_control_characters = true;
    assert_eq!(is_escape(ch, including_control_characters), true);
}

#[test]
fn test_is_escape_with_zero_control_character_excluding() {
    let ch = 0x00; // Null character
    let including_control_characters = false;
    assert_eq!(is_escape(ch, including_control_characters), false);
}

