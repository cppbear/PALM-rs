// Answer 0

#[test]
fn test_is_escape_quotes() {
    let result = is_escape(b'"', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_escape_backslash() {
    let result = is_escape(b'\\', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_escape_control_character_including() {
    let result = is_escape(0x0F, true);
    assert_eq!(result, true);
}

#[test]
fn test_is_escape_control_character_not_including() {
    let result = is_escape(0x0F, false);
    assert_eq!(result, false);
}

#[test]
fn test_is_escape_non_control_character_including() {
    let result = is_escape(b'a', true);
    assert_eq!(result, false);
}

#[test]
fn test_is_escape_non_control_character_not_including() {
    let result = is_escape(b'a', false);
    assert_eq!(result, false);
}

