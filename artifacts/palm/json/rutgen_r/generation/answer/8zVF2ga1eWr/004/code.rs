// Answer 0

#[test]
fn test_is_escape_case_quota() {
    let result = is_escape(b'"', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_escape_case_backslash() {
    let result = is_escape(b'\\', false);
    assert_eq!(result, true);
}

#[test]
fn test_is_escape_case_control_character_without_including() {
    let result = is_escape(0x01, false);
    assert_eq!(result, false);
}

#[test]
fn test_is_escape_case_non_control_character_without_including() {
    let result = is_escape(b'a', false);
    assert_eq!(result, false);
}

#[test]
fn test_is_escape_case_control_character_with_including() {
    let result = is_escape(0x01, true);
    assert_eq!(result, true);
}

#[test]
fn test_is_escape_case_high_value_with_including() {
    let result = is_escape(0x20, true);
    assert_eq!(result, false);
}

