// Answer 0

#[test]
fn test_description_repetition_count_unclosed() {
    let error = Error {
        kind: ErrorKind::RepetitionCountUnclosed,
        pattern: String::from("a{3"),
        span: Span {
            start: Position(1),
            end: Position(3),
        },
    };
    let _ = error.description();
}

#[test]
fn test_description_another_repetition_count_unclosed() {
    let error = Error {
        kind: ErrorKind::RepetitionCountUnclosed,
        pattern: String::from("b{5"),
        span: Span {
            start: Position(0),
            end: Position(3),
        },
    };
    let _ = error.description();
}

