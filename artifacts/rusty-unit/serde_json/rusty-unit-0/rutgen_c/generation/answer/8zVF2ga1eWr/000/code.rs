// Answer 0

#[test]
fn test_is_escape_double_quote() {
    assert!(is_escape(b'"', false));
}

#[test]
fn test_is_escape_backslash() {
    assert!(is_escape(b'\\', false));
}

#[test]
fn test_is_escape_control_character_when_including() {
    assert!(is_escape(0x1F, true));
}

#[test]
fn test_is_escape_control_character_when_not_including() {
    assert!(!is_escape(0x1F, false));
}

#[test]
fn test_is_escape_non_control_character() {
    assert!(!is_escape(b'a', true));
    assert!(!is_escape(b'a', false));
}

