// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

struct IdentFragment {
    span_data: Span,
}

impl IdentFragment {
    fn span(&self) -> Span {
        self.span_data.clone()
    }
    
    fn span(&self) -> Option<Span> {
        Some(self.span())
    }
}

#[test]
fn test_span_returns_some() {
    let span_data = Span { start: 0, end: 5 };
    let ident_fragment = IdentFragment { span_data };
    assert!(ident_fragment.span().is_some());
}

#[test]
fn test_span_content() {
    let span_data = Span { start: 1, end: 10 };
    let ident_fragment = IdentFragment { span_data };
    let span_option = ident_fragment.span();
    assert_eq!(span_option.unwrap(), Span { start: 1, end: 10 });
}

