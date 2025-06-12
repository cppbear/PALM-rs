// Answer 0

#[test]
fn test_case_insensitive_true() {
    let flags = Flags {
        case_insensitive: Some(true),
        ..Default::default()
    };
    flags.case_insensitive();
}

#[test]
fn test_case_insensitive_false() {
    let flags = Flags {
        case_insensitive: Some(false),
        ..Default::default()
    };
    flags.case_insensitive();
}

#[test]
fn test_case_insensitive_none() {
    let flags = Flags {
        case_insensitive: None,
        ..Default::default()
    };
    flags.case_insensitive();
}

