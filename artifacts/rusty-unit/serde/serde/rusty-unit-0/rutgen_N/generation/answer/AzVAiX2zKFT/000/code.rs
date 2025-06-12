// Answer 0

#[derive(Debug)]
struct IgnoredAny;

struct Visitor;

impl Visitor {
    fn visit_u128<E>(self, x: u128) -> Result<IgnoredAny, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
}

#[test]
fn test_visit_u128() {
    let visitor = Visitor;
    let result: Result<IgnoredAny, &str> = visitor.visit_u128(123456789012345678901234567890u128);
    assert!(result.is_ok());
}

#[test]
fn test_visit_u128_boundary() {
    let visitor = Visitor;
    let result: Result<IgnoredAny, &str> = visitor.visit_u128(0u128);
    assert!(result.is_ok());

    let result_max: Result<IgnoredAny, &str> = visitor.visit_u128(u128::MAX);
    assert!(result_max.is_ok());
}

