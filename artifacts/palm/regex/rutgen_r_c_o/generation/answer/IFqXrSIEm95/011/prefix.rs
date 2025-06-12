// Answer 0

#[test]
fn test_description_group_name_invalid() {
    let error = Error {
        kind: ErrorKind::GroupNameInvalid,
        pattern: String::from("(?P<invalid capture>abc)"),
        span: Span {
            start: Position::new(0),
            end: Position::new(18),
        },
    };
    error.description();
}

#[test]
fn test_description_group_name_empty() {
    let error = Error {
        kind: ErrorKind::GroupNameEmpty,
        pattern: String::from("(?P<>abc)"),
        span: Span {
            start: Position::new(0),
            end: Position::new(10),
        },
    };
    error.description();
}

#[test]
fn test_description_group_name_duplicate() {
    let error = Error {
        kind: ErrorKind::GroupNameDuplicate {
            original: Span {
                start: Position::new(0),
                end: Position::new(10),
            },
        },
        pattern: String::from("(?P<duplicate>abc)(?P<duplicate>def)"),
        span: Span {
            start: Position::new(0),
            end: Position::new(40),
        },
    };
    error.description();
}

