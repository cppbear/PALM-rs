// Answer 0

#[test]
fn test_deserialize_bytes_with_empty_array() {
    let value = Value::Array(vec![]);
    let result = value.deserialize_bytes(TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_bytes_with_array_of_strings() {
    let value = Value::Array(vec![
        Value::String(String::from("item1")),
        Value::String(String::from("item2")),
    ]);
    let result = value.deserialize_bytes(TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_bytes_with_mixed_values() {
    let value = Value::Array(vec![
        Value::String(String::from("string")),
        Value::Number(Number::from(42)),
    ]);
    let result = value.deserialize_bytes(TestVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_bytes_with_single_empty_string() {
    let value = Value::Array(vec![Value::String(String::from(""))]);
    let result = value.deserialize_bytes(TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_bytes_with_non_string_value() {
    let value = Value::Array(vec![Value::Bool(true)]);
    let result = value.deserialize_bytes(TestVisitor);
    assert!(result.is_err());
}

struct TestVisitor;

impl<'de> Visitor<'de> for TestVisitor {
    type Value = ();

    fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        Ok(())
    }

    // Other visitor methods can be implemented as no-op if not used
    forward_to_deserialize_any!{
        bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64,
        char, str, string, bytes, byte_buf, option, unit, 
        unit_struct, newtype_struct, seq, tuple, tuple_struct, 
        map, struct, enum, identifier, ignored_any
    }
}

