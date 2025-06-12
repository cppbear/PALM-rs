// Answer 0

#[test]
fn test_dot_matches_new_line_default() {
    let flags = Flags::default();
    assert_eq!(flags.dot_matches_new_line(), false);
}

#[test]
fn test_dot_matches_new_line_some_true() {
    let mut flags = Flags {
        dot_matches_new_line: Some(true),
        ..Flags::default()
    };
    assert_eq!(flags.dot_matches_new_line(), true);
}

#[test]
fn test_dot_matches_new_line_some_false() {
    let mut flags = Flags {
        dot_matches_new_line: Some(false),
        ..Flags::default()
    };
    assert_eq!(flags.dot_matches_new_line(), false);
}

#[test]
fn test_dot_matches_new_line_none() {
    let flags = Flags {
        dot_matches_new_line: None,
        ..Flags::default()
    };
    assert_eq!(flags.dot_matches_new_line(), false);
}

