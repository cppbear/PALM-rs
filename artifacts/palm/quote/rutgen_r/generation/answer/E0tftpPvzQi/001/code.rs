// Answer 0

#[derive(Debug)]
struct Span;

trait T {
    fn span(&self) -> Option<Span>;
}

struct TestStruct;

impl T for TestStruct {
    fn span(&self) -> Option<Span> {
        Some(Span)
    }
}

#[test]
fn test_span_some() {
    let test_struct = TestStruct;
    let result = test_struct.span();
    assert!(result.is_some());
}

#[test]
fn test_span_none() {
    struct NoneTestStruct;

    impl T for NoneTestStruct {
        fn span(&self) -> Option<Span> {
            None
        }
    }

    let none_test_struct = NoneTestStruct;
    let result = none_test_struct.span();
    assert!(result.is_none());
}

