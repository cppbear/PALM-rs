// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct ErrorType {
    span: Span,
}

impl ErrorType {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[test]
fn test_span() {
    let span = Span { start: 5, end: 10 };
    let error = ErrorType { span };
    
    let result = error.span();
    
    assert_eq!(result.start, 5);
    assert_eq!(result.end, 10);
}

#[test]
fn test_span_empty() {
    let span = Span { start: 0, end: 0 };
    let error = ErrorType { span };
    
    let result = error.span();
    
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 0);
}

