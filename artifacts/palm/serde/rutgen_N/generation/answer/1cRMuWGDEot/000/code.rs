// Answer 0

#[derive(Debug)]
struct IgnoredAny;

struct TestVisitor;

impl TestVisitor {
    fn visit_i128<E>(self, x: i128) -> Result<IgnoredAny, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
}

#[test]
fn test_visit_i128_positive() {
    let visitor = TestVisitor;
    let result = visitor.visit_i128(123456789012345678901234567890i128);
    assert!(result.is_ok());
}

#[test]
fn test_visit_i128_negative() {
    let visitor = TestVisitor;
    let result = visitor.visit_i128(-123456789012345678901234567890i128);
    assert!(result.is_ok());
}

#[test]
fn test_visit_i128_zero() {
    let visitor = TestVisitor;
    let result = visitor.visit_i128(0i128);
    assert!(result.is_ok());
}

