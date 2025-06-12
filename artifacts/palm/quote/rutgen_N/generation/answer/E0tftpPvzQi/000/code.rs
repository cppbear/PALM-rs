// Answer 0

#[derive(Default)]
struct TestStruct;

impl TestTrait for TestStruct {
    fn span(&self) -> Option<Span> {
        Some(Span::new(0, 1))
    }
}

#[test]
fn test_span_returns_some() {
    let test_instance = TestStruct::default();
    let result = test_instance.span();
    assert!(result.is_some());
}

#[test]
fn test_span_returns_correct_span() {
    let test_instance = TestStruct::default();
    let result = test_instance.span();
    assert_eq!(result, Some(Span::new(0, 1)));
}

#[derive(Default)]
struct EmptyStruct;

impl TestTrait for EmptyStruct {
    fn span(&self) -> Option<Span> {
        None
    }
}

#[test]
fn test_span_returns_none() {
    let empty_instance = EmptyStruct::default();
    let result = empty_instance.span();
    assert!(result.is_none());
}

