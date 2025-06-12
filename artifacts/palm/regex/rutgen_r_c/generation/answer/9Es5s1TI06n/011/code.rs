// Answer 0

#[test]
fn test_fmt_group_name_invalid() {
    let error = ErrorKind::GroupNameInvalid;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid capture group character");
}

#[test]
fn test_fmt_group_name_empty() {
    let error = ErrorKind::GroupNameEmpty;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "empty capture group name");
}

#[test]
fn test_fmt_group_name_duplicate() {
    let error = ErrorKind::GroupNameDuplicate {
        original: Span { start: Position::from(0), end: Position::from(5) }
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "duplicate capture group name");
}

