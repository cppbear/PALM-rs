// Answer 0

fn test_merge_all_none() {
    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }
    
    let mut current_flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    
    let previous_flags = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    current_flags.merge(&previous_flags);

    assert_eq!(current_flags.case_insensitive, Some(true));
    assert_eq!(current_flags.multi_line, Some(false));
    assert_eq!(current_flags.dot_matches_new_line, Some(true));
    assert_eq!(current_flags.swap_greed, Some(false));
    assert_eq!(current_flags.unicode, Some(true));
}

fn test_merge_some_values() {
    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    let mut current_flags = Flags {
        case_insensitive: Some(false),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: Some(true),
        unicode: None,
    };

    let previous_flags = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    current_flags.merge(&previous_flags);

    assert_eq!(current_flags.case_insensitive, Some(false)); // should remain the same
    assert_eq!(current_flags.multi_line, Some(false)); // should merge from previous
    assert_eq!(current_flags.dot_matches_new_line, Some(true)); // should merge from previous
    assert_eq!(current_flags.swap_greed, Some(true)); // should remain the same
    assert_eq!(current_flags.unicode, Some(true)); // should merge from previous
}

fn test_merge_partial_none() {
    struct Flags {
        case_insensitive: Option<bool>,
        multi_line: Option<bool>,
        dot_matches_new_line: Option<bool>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    }

    let mut current_flags = Flags {
        case_insensitive: None,
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: Some(false),
    };

    let previous_flags = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    current_flags.merge(&previous_flags);

    assert_eq!(current_flags.case_insensitive, Some(true)); // should merge from previous
    assert_eq!(current_flags.multi_line, Some(true)); // should remain the same
    assert_eq!(current_flags.dot_matches_new_line, Some(true)); // should merge from previous
    assert_eq!(current_flags.swap_greed, Some(false)); // should merge from previous
    assert_eq!(current_flags.unicode, Some(false)); // should remain the same
}

