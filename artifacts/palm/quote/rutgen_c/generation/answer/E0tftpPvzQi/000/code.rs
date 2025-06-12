// Answer 0

#[test]
fn test_span_for_cow_with_ident_fragment() {
    struct DummyIdentFragment;

    impl IdentFragment for DummyIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "DummyIdentFragment")
        }
        fn span(&self) -> Option<Span> {
            Some(Span::call_site())
        }
    }

    let fragment: Cow<DummyIdentFragment> = Cow::Borrowed(&DummyIdentFragment);
    assert!(fragment.span().is_some());
}

#[test]
fn test_span_for_empty_cow() {
    struct EmptyIdentFragment;

    impl IdentFragment for EmptyIdentFragment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "EmptyIdentFragment")
        }
        fn span(&self) -> Option<Span> {
            None
        }
    }

    let fragment: Cow<EmptyIdentFragment> = Cow::Owned(EmptyIdentFragment);
    assert!(fragment.span().is_none());
}

