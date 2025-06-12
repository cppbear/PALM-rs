// Answer 0

#[derive(Debug)]
struct IgnoredAny;

struct TestVisitor;

impl TestVisitor {
    fn visit_bool<E>(self, x: bool) -> Result<IgnoredAny, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
}

#[test]
fn test_visit_bool_true() {
    let visitor = TestVisitor;
    let result: Result<IgnoredAny, &str> = visitor.visit_bool(true);
    assert!(result.is_ok());
}

#[test]
fn test_visit_bool_false() {
    let visitor = TestVisitor;
    let result: Result<IgnoredAny, &str> = visitor.visit_bool(false);
    assert!(result.is_ok());
}

