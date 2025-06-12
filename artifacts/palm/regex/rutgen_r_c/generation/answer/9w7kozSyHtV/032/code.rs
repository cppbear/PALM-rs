// Answer 0

#[test]
fn test_merge_with_all_options_set() {
    let mut flags_a = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    let flags_b = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: Some(false),
    };

    flags_a.merge(&flags_b);
    
    assert_eq!(flags_a.case_insensitive, Some(true));
    assert_eq!(flags_a.multi_line, Some(false));
    assert_eq!(flags_a.dot_matches_new_line, Some(true));
    assert_eq!(flags_a.swap_greed, Some(false));
    assert_eq!(flags_a.unicode, Some(true));
}

#[test]
fn test_merge_with_none_values_on_self() {
    let mut flags_a = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    let flags_b = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    flags_a.merge(&flags_b);
    
    assert_eq!(flags_a.case_insensitive, Some(true));
    assert_eq!(flags_a.multi_line, Some(false));
    assert_eq!(flags_a.dot_matches_new_line, Some(true));
    assert_eq!(flags_a.swap_greed, Some(false));
    assert_eq!(flags_a.unicode, Some(true));
}

#[test]
fn test_merge_with_only_self_values_set() {
    let mut flags_a = Flags {
        case_insensitive: Some(true),
        multi_line: Some(true),
        dot_matches_new_line: Some(true),
        swap_greed: Some(true),
        unicode: Some(true),
    };

    let flags_b = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    flags_a.merge(&flags_b);
    
    assert_eq!(flags_a.case_insensitive, Some(true));
    assert_eq!(flags_a.multi_line, Some(true));
    assert_eq!(flags_a.dot_matches_new_line, Some(true));
    assert_eq!(flags_a.swap_greed, Some(true));
    assert_eq!(flags_a.unicode, Some(true));
}

#[test]
fn test_merge_with_partially_none_previous() {
    let mut flags_a = Flags {
        case_insensitive: None,
        multi_line: Some(false),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: None,
    };

    let flags_b = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: Some(false),
        unicode: Some(true),
    };

    flags_a.merge(&flags_b);
    
    assert_eq!(flags_a.case_insensitive, Some(true));
    assert_eq!(flags_a.multi_line, Some(false));
    assert_eq!(flags_a.dot_matches_new_line, Some(false));
    assert_eq!(flags_a.swap_greed, Some(true));
    assert_eq!(flags_a.unicode, Some(true));
}

