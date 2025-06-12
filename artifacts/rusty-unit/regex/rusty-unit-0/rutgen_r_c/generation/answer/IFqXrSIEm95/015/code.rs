// Answer 0

#[test]
fn test_description_flag_unexpected_eof() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Span {
        start: Position,
        end: Position,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        FlagUnexpectedEof,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Position {
        offset: usize,
    }

    let error_instance = Error {
        kind: ErrorKind::FlagUnexpectedEof,
        pattern: String::from("(?"),
        span: Span {
            start: Position { offset: 0 },
            end: Position { offset: 2 },
        },
    };

    assert_eq!(error_instance.description(), "unexpected eof (flag)");
}

#[test]
fn test_description_another_flag_unexpected_eof() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Span {
        start: Position,
        end: Position,
    }
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        FlagUnexpectedEof,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Position {
        offset: usize,
    }

    let error_instance = Error {
        kind: ErrorKind::FlagUnexpectedEof,
        pattern: String::from("(?i"),
        span: Span {
            start: Position { offset: 0 },
            end: Position { offset: 3 },
        },
    };

    assert_eq!(error_instance.description(), "unexpected eof (flag)");
}

