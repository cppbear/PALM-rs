// Answer 0

#[test]
fn test_deserialize_bool_with_null() {
    let value = Value::Null;
    let visitor = DummyVisitor {};
    value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_number() {
    let value = Value::Number(Number { n: 0 });
    let visitor = DummyVisitor {};
    value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_string() {
    let value = Value::String(String::from("not a bool"));
    let visitor = DummyVisitor {};
    value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_object() {
    let value = Value::Object(Map::new());
    let visitor = DummyVisitor {};
    value.deserialize_bool(visitor);
}

#[test]
fn test_deserialize_bool_with_array() {
    let value = Value::Array(vec![Value::Null]);
    let visitor = DummyVisitor {};
    value.deserialize_bool(visitor);
}

struct DummyVisitor;

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = ();
    
    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
        Ok(())
    }
    
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean")
    }
}

