// Answer 0

#[test]
fn test_is_escape_with_escape_character() {
    let ch = b'"';
    let including_control_characters = true;
    assert!(is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_with_backslash() {
    let ch = b'\\';
    let including_control_characters = false;
    assert!(is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_with_control_character() {
    let ch = b'\n'; // control character
    let including_control_characters = true;
    assert!(is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_with_non_escape_character() {
    let ch = b'a'; 
    let including_control_characters = false;
    assert!(!is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_with_non_escape_character_including_control() {
    let ch = b'a'; 
    let including_control_characters = true;
    assert!(!is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_with_lower_boundary_control_character() {
    let ch = 0x1F; // lowest control character (< 0x20)
    let including_control_characters = true;
    assert!(is_escape(ch, including_control_characters));
}

#[test]
fn test_is_escape_with_boundary_non_control_character() {
    let ch = 0x20; // boundary value, not a control character
    let including_control_characters = true;
    assert!(!is_escape(ch, including_control_characters));
}


