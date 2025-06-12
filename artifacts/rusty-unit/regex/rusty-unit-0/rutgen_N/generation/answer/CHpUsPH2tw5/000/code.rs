// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ErrorKind {
    SyntaxError,
    OtherError,
}

#[derive(Debug)]
struct Error {
    kind: ErrorKind,
    pattern: String,
    span: Span,
}

struct Pattern {
    pattern: String,
}

impl Pattern {
    fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error { kind: kind, pattern: self.pattern.to_string(), span: span }
    }
}

#[test]
fn test_error_creation_with_syntax_error() {
    let pattern = Pattern { pattern: String::from("a*b") };
    let span = Span { start: 0, end: 3 };
    
    let error = pattern.error(span.clone(), ErrorKind::SyntaxError);
    
    assert_eq!(error.kind, ErrorKind::SyntaxError);
    assert_eq!(error.pattern, "a*b");
    assert_eq!(error.span, span);
}

#[test]
fn test_error_creation_with_other_error() {
    let pattern = Pattern { pattern: String::from("a+b") };
    let span = Span { start: 1, end: 4 };
    
    let error = pattern.error(span.clone(), ErrorKind::OtherError);
    
    assert_eq!(error.kind, ErrorKind::OtherError);
    assert_eq!(error.pattern, "a+b");
    assert_eq!(error.span, span);
}

#[test]
fn test_error_creation_boundary_conditions() {
    let pattern = Pattern { pattern: String::from("") };
    let span = Span { start: 0, end: 0 };
    
    let error = pattern.error(span.clone(), ErrorKind::SyntaxError);
    
    assert_eq!(error.kind, ErrorKind::SyntaxError);
    assert_eq!(error.pattern, "");
    assert_eq!(error.span, span);
}

