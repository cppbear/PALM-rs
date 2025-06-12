// Answer 0

#[derive(Debug)]
struct IgnoredAny;

struct TestVisitor;

impl TestVisitor {
    fn visit_none<E>(self) -> Result<IgnoredAny, E> {
        Ok(IgnoredAny)
    }
}

#[test]
fn test_visit_none_success() {
    let visitor = TestVisitor;
    let result: Result<IgnoredAny, &'static str> = visitor.visit_none();
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_visit_none_return_value() {
    let visitor = TestVisitor;
    let result: Result<IgnoredAny, &'static str> = visitor.visit_none();
    assert!(result.unwrap() == IgnoredAny);
}

