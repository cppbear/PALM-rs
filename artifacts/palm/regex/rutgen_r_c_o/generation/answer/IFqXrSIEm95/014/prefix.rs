// Answer 0

#[test]
fn test_flag_unrecognized() {
    let error = Error {
        kind: ErrorKind::FlagUnrecognized,
        pattern: "some_pattern".to_string(),
        span: Span {
            start: Position(0),
            end: Position(1),
        },
    };
    let _ = error.description();
}

#[test]
fn test_flag_unrecognized_with_extra_data() {
    let error = Error {
        kind: ErrorKind::FlagUnrecognized,
        pattern: "another_pattern".to_string(),
        span: Span {
            start: Position(2),
            end: Position(3),
        },
    };
    let _ = error.description();
}

