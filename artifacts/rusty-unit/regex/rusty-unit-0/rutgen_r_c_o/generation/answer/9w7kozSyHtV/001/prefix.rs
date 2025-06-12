// Answer 0

#[test]
fn test_merge_flags_all_none() {
    let mut flags = Flags::default();
    let previous = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };
    flags.merge(&previous);
}

#[test]
fn test_merge_flags_some_values() {
    let mut flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    let previous = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: Some(false),
    };
    flags.merge(&previous);
}

#[test]
fn test_merge_flags_skip_none() {
    let mut flags = Flags {
        case_insensitive: None,
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: Some(false),
        unicode: None,
    };
    let previous = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: Some(true),
        swap_greed: None,
        unicode: Some(true),
    };
    flags.merge(&previous);
}

