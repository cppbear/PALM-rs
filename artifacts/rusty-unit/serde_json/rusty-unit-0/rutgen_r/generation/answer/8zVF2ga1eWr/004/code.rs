// Answer 0

fn is_escape(ch: u8, including_control_characters: bool) -> bool {
    ch == b'"' || ch == b'\\' || (including_control_characters && ch < 0x20)
}

#[test]
fn test_is_escape_double_quote() {
    let ch: u8 = b'"';
    let including_control_characters = false;
    assert!(is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_backslash() {
    let ch: u8 = b'\\';
    let including_control_characters = false;
    assert!(is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_control_character_including_false() {
    let ch: u8 = 0x1F; // control character less than 0x20
    let including_control_characters = false;
    assert!(!is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_non_control_character_including_false() {
    let ch: u8 = b'a'; // non-control character
    let including_control_characters = false;
    assert!(!is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_control_character_including_true() {
    let ch: u8 = 0x1F; // control character less than 0x20
    let including_control_characters = true;
    assert!(is_escape(ch, including_control_characters));
}

