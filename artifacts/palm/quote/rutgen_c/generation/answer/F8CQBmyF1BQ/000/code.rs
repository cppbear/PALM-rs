// Answer 0

#[test]
fn test_span_none() {
    struct TestStruct;

    impl IdentFragment for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStruct")
        }
    }

    let test_instance = TestStruct;
    assert_eq!(test_instance.span(), None);
}

#[test]
fn test_span_option_some() {
    struct TestStructWithSpan {
        span: Span,
    }

    impl IdentFragment for TestStructWithSpan {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestStructWithSpan")
        }

        fn span(&self) -> Option<Span> {
            Some(self.span)
        }
    }

    let test_instance = TestStructWithSpan {
        span: Span::call_site(),
    };
    assert!(test_instance.span().is_some());
}

