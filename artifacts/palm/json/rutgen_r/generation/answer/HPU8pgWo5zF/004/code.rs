// Answer 0

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

impl Value {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(v) => v.serialize(serializer),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Value::Object(m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Value::Object(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct TestSerializer {
    pub values: Vec<(String, Value)>,
}

impl serde::Serializer for TestSerializer {
    type Ok = ();
    type Error = ();

    // Required methods...

    fn serialize_unit(self) -> core::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_bool(self, _: bool) -> core::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_str(self, _: &str) -> core::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_map(self, _: Option<usize>) -> core::result::Result<impl serde::ser::SerializeMap + 'static, Self::Error> {
        Ok(TestMapSerializer)
    }
}

struct TestMapSerializer;

impl serde::ser::SerializeMap for TestMapSerializer {
    type Ok = ();
    type Error = ();

    fn serialize_entry<T1, T2>(&mut self, _: T1, _: T2) -> core::result::Result<(), Self::Error>
    where
        T1: serde::ser::Serialize,
        T2: serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> core::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[test]
fn test_serialize_object_with_empty_map() {
    let mut map = std::collections::HashMap::new();
    let value = Value::Object(map);
    let serializer = TestSerializer { values: vec![] };
    let result = value.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_object_with_single_entry() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), Value::Null);
    let value = Value::Object(map);
    let serializer = TestSerializer { values: vec![] };
    let result = value.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_object_with_multiple_entries() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(42));
    let value = Value::Object(map);
    let serializer = TestSerializer { values: vec![] };
    let result = value.serialize(serializer);
    assert!(result.is_ok());
}

