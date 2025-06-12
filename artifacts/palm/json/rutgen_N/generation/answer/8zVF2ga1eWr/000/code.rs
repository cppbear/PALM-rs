// Answer 0

#[test]
fn test_is_escape_double_quote() {
    assert_eq!(is_escape(b'"', false), true);
}

#[test]
fn test_is_escape_backslash() {
    assert_eq!(is_escape(b'\\', false), true);
}

#[test]
fn test_is_escape_control_character_including() {
    assert_eq!(is_escape(0x1F, true), true);
}

#[test]
fn test_is_escape_control_character_not_including() {
    assert_eq!(is_escape(0x1F, false), false);
}

#[test]
fn test_is_escape_non_control_character() {
    assert_eq!(is_escape(b'a', false), false);
}

