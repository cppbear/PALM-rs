// Answer 0

#[test]
fn test_case_insensitive_with_some_true() {
    let flags = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    assert!(flags.case_insensitive());
}

#[test]
fn test_case_insensitive_with_some_false() {
    let flags = Flags {
        case_insensitive: Some(false),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    assert!(!flags.case_insensitive());
}

#[test]
fn test_case_insensitive_with_none() {
    let flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    assert!(!flags.case_insensitive());
}

