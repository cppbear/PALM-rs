// Answer 0

#[derive(Debug)]
struct Span;

struct IdentFragment;

impl IdentFragment {
    fn span(&self) -> Option<Span> {
        None
    }
}

#[test]
fn test_span_returns_none() {
    let ident_fragment = IdentFragment;
    assert_eq!(ident_fragment.span(), None);
}

