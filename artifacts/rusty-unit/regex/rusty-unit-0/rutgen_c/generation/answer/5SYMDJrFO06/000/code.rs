// Answer 0

#[test]
fn test_multi_line_with_none() {
    let flags = Flags::default();
    assert_eq!(flags.multi_line(), false);
}

#[test]
fn test_multi_line_with_some_false() {
    let flags = Flags {
        multi_line: Some(false),
        ..Flags::default()
    };
    assert_eq!(flags.multi_line(), false);
}

#[test]
fn test_multi_line_with_some_true() {
    let flags = Flags {
        multi_line: Some(true),
        ..Flags::default()
    };
    assert_eq!(flags.multi_line(), true);
}

