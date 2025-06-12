// Answer 0

#[test]
fn test_case_insensitive_with_none() {
    let flags = Flags::default();
    assert_eq!(flags.case_insensitive(), false);
}

#[test]
fn test_case_insensitive_with_false() {
    let flags = Flags {
        case_insensitive: Some(false),
        ..Flags::default()
    };
    assert_eq!(flags.case_insensitive(), false);
}

#[test]
fn test_case_insensitive_with_true() {
    let flags = Flags {
        case_insensitive: Some(true),
        ..Flags::default()
    };
    assert_eq!(flags.case_insensitive(), true);
}

