// Answer 0

#[derive(Debug)]
struct TestError;

impl serde::de::Error for TestError {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        println!("Custom error: {}", msg);
        TestError
    }
}

struct TestVisitor;

impl serde::Deserializer for TestVisitor {
    type Error = TestError;

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> 
    where
        V: serde::de::Visitor,
    {
        visitor.visit_unit()
    }
}

#[test]
fn test_visit_unit() {
    let visitor = TestVisitor;
    let result: Result<(), TestError> = visitor.deserialize_unit(serde::de::impls::UnitVisitor);
    assert_eq!(result, Ok(()));
}

