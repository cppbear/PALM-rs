// Answer 0

#[derive(Debug)]
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
fn test_merge_with_all_none() {
    let mut current_flags = Flags::new();
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

#[test]
fn test_merge_with_some_values() {
    let mut current_flags = Flags {
        case_insensitive: Some(false),
        multi_line: Some(true),
        dot_matches_new_line: Some(false),
        swap_greed: Some(true),
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

    assert_eq!(current_flags.case_insensitive, Some(false)); // should remain Some(false)
    assert_eq!(current_flags.multi_line, Some(true)); // should remain Some(true)
    assert_eq!(current_flags.dot_matches_new_line, Some(false)); // should remain Some(false)
    assert_eq!(current_flags.swap_greed, Some(true)); // should remain Some(true)
    assert_eq!(current_flags.unicode, Some(false)); // should remain Some(false)
}

#[test]
fn test_merge_with_all_initial_values() {
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

    assert_eq!(current_flags.case_insensitive, Some(false)); // should remain Some(false)
    assert_eq!(current_flags.multi_line, Some(false)); // should remain Some(false)
    assert_eq!(current_flags.dot_matches_new_line, Some(false)); // should remain Some(false)
    assert_eq!(current_flags.swap_greed, Some(false)); // should remain Some(false)
    assert_eq!(current_flags.unicode, Some(false)); // should remain Some(false)
}

