// Answer 0

#[test]
fn test_error_span() {
    struct TestError {
        span: Span,
    }
    
    impl Error {
        fn new(span: Span) -> Self {
            Self {
                kind: ErrorKind::UnicodeNotAllowed,
                pattern: String::from("test pattern"),
                span,
            }
        }
    }

    let position_start = Position { value: 0 };
    let position_end = Position { value: 10 };
    let span = Span { start: position_start, end: position_end };
    let error = TestError::new(span.clone());

    assert_eq!(error.span(), &span);
}

#[test]
fn test_error_span_with_empty_start() {
    struct TestError {
        span: Span,
    }
    
    impl Error {
        fn new(span: Span) -> Self {
            Self {
                kind: ErrorKind::UnicodeNotAllowed,
                pattern: String::from("test pattern"),
                span,
            }
        }
    }

    let position_start = Position { value: 0 };
    let position_end = Position { value: 0 };
    let span = Span { start: position_start, end: position_end };
    let error = TestError::new(span.clone());

    assert_eq!(error.span(), &span);
}

#[test]
fn test_error_span_with_same_start_end() {
    struct TestError {
        span: Span,
    }
    
    impl Error {
        fn new(span: Span) -> Self {
            Self {
                kind: ErrorKind::UnicodeNotAllowed,
                pattern: String::from("test pattern"),
                span,
            }
        }
    }

    let position_start = Position { value: 5 };
    let span = Span { start: position_start, end: position_start };
    let error = TestError::new(span.clone());

    assert_eq!(error.span(), &span);
}

