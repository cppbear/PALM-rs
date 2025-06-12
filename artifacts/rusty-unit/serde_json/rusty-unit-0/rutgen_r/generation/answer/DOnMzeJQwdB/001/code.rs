// Answer 0

#[derive(Debug)]
struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_bool(self, value: bool) -> Result<Self::Value, Error> {
        Ok(())
    }
}

#[test]
fn test_deserialize_bool_invalid_type_not_bool() {
    // Given a Value that does not match Value::Bool
    let value = Value::String("not a bool".to_string());
    let visitor = MockVisitor;

    // When calling deserialize_bool
    let result = value.deserialize_bool(visitor);

    // Then it should return an error
    assert!(result.is_err());
}

#[test]
fn test_deserialize_bool_invalid_type_integer() {
    // Given a Value that does not match Value::Bool
    let value = Value::Number(42.0.into());
    let visitor = MockVisitor;

    // When calling deserialize_bool
    let result = value.deserialize_bool(visitor);

    // Then it should return an error
    assert!(result.is_err());
}

#[test]
fn test_deserialize_bool_invalid_type_null() {
    // Given a Value that does not match Value::Bool
    let value = Value::Null;
    let visitor = MockVisitor;

    // When calling deserialize_bool
    let result = value.deserialize_bool(visitor);

    // Then it should return an error
    assert!(result.is_err());
}

