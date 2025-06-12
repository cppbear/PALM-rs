// Answer 0

#[derive(Debug, Clone)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
}

impl Flags {
    fn new() -> Self {
        Flags {
            case_insensitive: None,
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }
    }
}

#[test]
fn test_merge_case_insensitive_set() {
    let mut current = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };
    
    let previous = Flags {
        case_insensitive: Some(true),
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    current.merge(&previous);
    assert_eq!(current.case_insensitive, Some(true));
}

#[test]
fn test_merge_multi_line_set() {
    let mut current = Flags::new();
    
    let previous = Flags {
        case_insensitive: None,
        multi_line: Some(true),
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: None,
    };

    current.merge(&previous);
    assert_eq!(current.multi_line, Some(true));
}

#[test]
fn test_merge_dot_matches_new_line_set() {
    let mut current = Flags::new();
    
    let previous = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: Some(true),
        swap_greed: None,
        unicode: None,
    };

    current.merge(&previous);
    assert_eq!(current.dot_matches_new_line, Some(true));
}

#[test]
fn test_merge_swap_greed_set() {
    let mut current = Flags::new();
    
    let previous = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: Some(true),
        unicode: None,
    };

    current.merge(&previous);
    assert_eq!(current.swap_greed, Some(true));
}

#[test]
fn test_merge_unicode_set() {
    let mut current = Flags::new();
    
    let previous = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: None,
        swap_greed: None,
        unicode: Some(true),
    };

    current.merge(&previous);
    assert_eq!(current.unicode, Some(true));
}

#[test]
fn test_merge_no_previous_flags() {
    let mut current = Flags::new();
    
    let previous = Flags::new(); // All None, nothing to merge

    current.merge(&previous);
    assert_eq!(current, previous); // Should still be all None
}

