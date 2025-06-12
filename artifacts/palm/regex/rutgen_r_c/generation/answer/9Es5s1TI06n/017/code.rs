// Answer 0

#[test]
fn test_error_kind_display_flag_duplicate() {
    struct TestSpan {
        start: Position,
        end: Position,
    }

    let span = TestSpan {
        start: Position { /* initialize Position fields */ },
        end: Position { /* initialize Position fields */ },
    };

    let error = ErrorKind::FlagDuplicate { original: Span { start: span.start, end: span.end } };
    let result = format!("{}", error);
    assert_eq!(result, "duplicate flag");
}

#[test]
fn test_error_kind_display_flag_duplicate_with_span() {
    struct TestSpan {
        start: Position,
        end: Position,
    }

    let span1 = TestSpan {
        start: Position { /* initialize Position fields */ },
        end: Position { /* initialize Position fields */ },
    };

    let error = ErrorKind::FlagDuplicate { original: Span { start: span1.start, end: span1.end } };
    let result = format!("{}", error);
    assert_eq!(result, "duplicate flag");
}

