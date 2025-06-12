// Answer 0

#[test]
fn test_merge_with_all_fields_set() {
    let mut current_flags = Flags {
        case_insensitive: Some(false),
        multi_line: Some(false),
        dot_matches_new_line: Some(false),
        swap_greed: Some(false),
        unicode: Some(false),
    };
    
    let previous_flags = Flags {
        case_insensitive: Some(true),
        multi_line: Some(true),
        dot_matches_new_line: Some(true),
        swap_greed: Some(true),
        unicode: Some(true),
    };

    current_flags.merge(&previous_flags);
}

#[test]
fn test_merge_with_some_fields_none() {
    let mut current_flags = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: Some(false),
        swap_greed: None,
        unicode: Some(true),
    };
    
    let previous_flags = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(true),
        swap_greed: Some(false),
        unicode: None,
    };

    current_flags.merge(&previous_flags);
}

#[test]
fn test_merge_with_all_none_fields() {
    let mut current_flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    
    let previous_flags = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: Some(false),
    };

    current_flags.merge(&previous_flags);
} 

#[test]
fn test_merge_with_no_previous_fields() {
    let mut current_flags = Flags {
        case_insensitive: Some(true),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
        unicode: Some(false),
    };
    
    let previous_flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    current_flags.merge(&previous_flags);
} 

#[test]
fn test_merge_with_mixed_flags() {
    let mut current_flags = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: Some(false),
        unicode: Some(true),
    };
    
    let previous_flags = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: Some(true),
        swap_greed: None,
        unicode: Some(false),
    };

    current_flags.merge(&previous_flags);
}

