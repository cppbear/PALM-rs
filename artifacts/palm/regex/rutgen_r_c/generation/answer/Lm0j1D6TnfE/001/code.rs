// Answer 0

#[test]
fn test_dot_matches_new_line_with_some_value_true() {
    let flags = Flags {
        dot_matches_new_line: Some(true),
        ..Flags::default()
    };
    assert_eq!(flags.dot_matches_new_line(), true);
}

#[test]
fn test_dot_matches_new_line_with_some_value_false() {
    let flags = Flags {
        dot_matches_new_line: Some(false),
        ..Flags::default()
    };
    assert_eq!(flags.dot_matches_new_line(), false);
}

#[test]
fn test_dot_matches_new_line_with_none() {
    let flags = Flags::default();
    assert_eq!(flags.dot_matches_new_line(), false);
}

