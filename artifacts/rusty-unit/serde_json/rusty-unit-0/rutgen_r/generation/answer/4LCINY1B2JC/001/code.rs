// Answer 0

#[derive(Debug)]
enum Value {
    String(String),
    Number(i32),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_string(self, value: String) -> Result<Self::Value, Error> {
        Ok(())
    }

    // Add any required methods from Visitor trait here...
}

#[derive(Debug)]
struct Error;

impl Value {
    fn invalid_type<V>(&self, _visitor: &V) -> Error {
        Error
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(v) => visitor.visit_string(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

#[test]
fn test_deserialize_string_non_string_value() {
    let value = Value::Number(10);
    let visitor = MockVisitor;

    let result = value.deserialize_string(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_string_null_value() {
    let value = Value::Null;
    let visitor = MockVisitor;

    let result = value.deserialize_string(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_string_boolean_value() {
    let value = Value::Boolean(true);
    let visitor = MockVisitor;

    let result = value.deserialize_string(visitor);
    assert!(result.is_err());
}

