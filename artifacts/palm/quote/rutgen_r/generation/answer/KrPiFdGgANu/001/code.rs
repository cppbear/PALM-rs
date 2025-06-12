// Answer 0

#[test]
fn test_span_valid() {
    struct Dummy {
        span_value: Span,
    }

    impl Dummy {
        fn span(&self) -> Span {
            self.span_value
        }

        fn new(span_value: Span) -> Self {
            Dummy { span_value }
        }
    }

    let expected_span = Span::new(0, 10); // Example span where start=0 and end=10
    let dummy = Dummy::new(expected_span.clone());

    assert_eq!(dummy.span(), expected_span);
    assert_eq!(dummy.span().span(), Some(expected_span));
}

#[test]
fn test_span_no_value() {
    struct EmptyDummy;

    impl EmptyDummy {
        fn span(&self) -> Span {
            panic!("No span available");
        }
    }

    let empty_dummy = EmptyDummy;

    #[should_panic(expected = "No span available")]
    {
        let _ = empty_dummy.span();
    }
}

