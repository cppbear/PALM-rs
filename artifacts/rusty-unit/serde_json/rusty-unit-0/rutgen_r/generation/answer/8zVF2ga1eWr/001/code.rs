// Answer 0

#[test]
fn test_is_escape_not_double_quote() {
    assert_eq!(is_escape(b'a', false), false);
    assert_eq!(is_escape(b'a', true), false);
}

#[test]
fn test_is_escape_backslash() {
    assert_eq!(is_escape(b'\\', false), true);
    assert_eq!(is_escape(b'\\', true), true);
}

#[test]
fn test_is_escape_control_characters() {
    assert_eq!(is_escape(0x0, true), true); // Null character
    assert_eq!(is_escape(0x1F, true), true); // Unit separator
    assert_eq!(is_escape(0x20, true), false); // Space character
}

#[test]
fn test_is_escape_non_control_characters() {
    assert_eq!(is_escape(b'a', true), false);
    assert_eq!(is_escape(b'z', true), false);
} 

#[test]
fn test_is_escape_beyond_boundaries() {
    assert_eq!(is_escape(0x00, true), true); // Lower boundary control character
    assert_eq!(is_escape(0x1F, true), true); // Upper boundary control character
    assert_eq!(is_escape(0x20, true), false); // The boundary value
    assert_eq!(is_escape(0x21, true), false); // Just above boundary
} 

#[test]
fn test_is_escape_with_including_control_characters_false() {
    assert_eq!(is_escape(0x0, false), false); // Null character
    assert_eq!(is_escape(0x1F, false), false); // Unit separator
    assert_eq!(is_escape(0x20, false), false); // Space character
} 

#[test]
fn test_is_escape_direct_panics() {
    assert_eq!(is_escape(b'"', true), true); // Direct check for double quote
    assert_eq!(is_escape(b'"', false), true); // Double quote should return true regardless of including_control_characters
}

