// Answer 0

#[test]
fn test_merge_all_flags_none() {
    let mut flags1 = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    
    let flags2 = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    flags1.merge(&flags2);

    assert_eq!(flags1.case_insensitive, Some(true));
    assert_eq!(flags1.multi_line, Some(false));
    assert_eq!(flags1.dot_matches_new_line, Some(true));
    assert_eq!(flags1.swap_greed, Some(false));
    assert_eq!(flags1.unicode, Some(true));
}

#[test]
fn test_merge_partial_flags_none() {
    let mut flags1 = Flags {
        case_insensitive: None,
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    
    let flags2 = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: Some(true),
    };

    flags1.merge(&flags2);

    assert_eq!(flags1.case_insensitive, Some(true));
    assert_eq!(flags1.multi_line, Some(true));
    assert_eq!(flags1.dot_matches_new_line, Some(true));
    assert_eq!(flags1.swap_greed, Some(false));
    assert_eq!(flags1.unicode, Some(true));
}

#[test]
fn test_merge_all_flags_set() {
    let mut flags1 = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: Some(false),
    };
    
    let flags2 = Flags {
        case_insensitive: Some(true),
        multi_line: Some(false),
        dot_matches_new_line: Some(true),
        swap_greed: None,
        unicode: Some(true),
    };

    flags1.merge(&flags2);

    assert_eq!(flags1.case_insensitive, Some(false));
    assert_eq!(flags1.multi_line, Some(true));
    assert_eq!(flags1.dot_matches_new_line, Some(false));
    assert_eq!(flags1.swap_greed, Some(true));
    assert_eq!(flags1.unicode, Some(false));
}

