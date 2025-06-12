// Answer 0

#[test]
fn test_span_return_none() {
    struct IdentFragment;

    impl IdentFragment {
        fn span(&self) -> Option<Span> {
            None
        }
    }
    
    let fragment = IdentFragment;
    assert_eq!(fragment.span(), None);
}

