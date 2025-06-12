// Answer 0

#[derive(Debug, PartialEq)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {
    SyntaxError,
    OtherError,
}

#[derive(Debug, PartialEq)]
struct Error {
    kind: ErrorKind,
    pattern: String,
    span: Span,
}

struct PatternWrapper {
    pattern: String,
}

impl PatternWrapper {
    fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error { kind: kind, pattern: self.pattern.to_string(), span: span }
    }
}

#[test]
fn test_error_syntax_error() {
    let wrapper = PatternWrapper {
        pattern: String::from("test_pattern"),
    };
    let span = Span { start: 0, end: 12 };
    let result = wrapper.error(span, ErrorKind::SyntaxError);
    
    assert_eq!(result, Error {
        kind: ErrorKind::SyntaxError,
        pattern: String::from("test_pattern"),
        span: Span { start: 0, end: 12 },
    });
}

#[test]
fn test_error_other_error() {
    let wrapper = PatternWrapper {
        pattern: String::from("another_pattern"),
    };
    let span = Span { start: 5, end: 20 };
    let result = wrapper.error(span, ErrorKind::OtherError);
    
    assert_eq!(result, Error {
        kind: ErrorKind::OtherError,
        pattern: String::from("another_pattern"),
        span: Span { start: 5, end: 20 },
    });
}

#[test]
fn test_error_boundary_conditions() {
    let wrapper = PatternWrapper {
        pattern: String::from("boundary_test"),
    };
    
    // Testing with zero-length span
    let span_zero = Span { start: 5, end: 5 };
    let result_zero = wrapper.error(span_zero, ErrorKind::SyntaxError);
    
    assert_eq!(result_zero, Error {
        kind: ErrorKind::SyntaxError,
        pattern: String::from("boundary_test"),
        span: Span { start: 5, end: 5 },
    });

    // Testing with maximum length span
    let span_max = Span { start: 0, end: usize::MAX };
    let result_max = wrapper.error(span_max, ErrorKind::OtherError);
    
    assert_eq!(result_max, Error {
        kind: ErrorKind::OtherError,
        pattern: String::from("boundary_test"),
        span: Span { start: 0, end: usize::MAX },
    });
}

