// Answer 0

#[derive(Debug)]
struct IgnoredAny;

struct DummyVisitor;

impl DummyVisitor {
    fn visit_i64<E>(self, x: i64) -> Result<IgnoredAny, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
}

#[test]
fn test_visit_i64_positive() {
    let visitor = DummyVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_i64(42);
    assert!(result.is_ok());
}

#[test]
fn test_visit_i64_negative() {
    let visitor = DummyVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_i64(-1);
    assert!(result.is_ok());
}

#[test]
fn test_visit_i64_zero() {
    let visitor = DummyVisitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_i64(0);
    assert!(result.is_ok());
}

