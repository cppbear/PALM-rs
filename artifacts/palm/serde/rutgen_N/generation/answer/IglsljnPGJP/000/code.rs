// Answer 0

#[derive(Debug)]
struct MyVisitor;

impl MyVisitor {
    type Value = i32;

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.parse().map_err(E::custom)
    }
}

#[test]
fn test_visit_str_valid_input() {
    let visitor = MyVisitor;
    let result: Result<i32, _> = visitor.visit_str("42");
    assert_eq!(result, Ok(42));
}

#[test]
fn test_visit_str_invalid_input() {
    let visitor = MyVisitor;
    let result: Result<i32, _> = visitor.visit_str("invalid");
    assert!(result.is_err());
}

