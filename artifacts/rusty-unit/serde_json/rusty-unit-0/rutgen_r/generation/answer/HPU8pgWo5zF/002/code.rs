// Answer 0

#[derive(Debug, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::HashMap<String, Value>),
}

impl Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(v) => v.serialize(serializer),
            Value::Object(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

#[derive(Debug)]
struct MockSerializer {
    map_entries: Vec<(String, Value)>,
    serialize_map_called: bool,
}

impl serde::Serializer for MockSerializer {
    type Ok = ();
    type Error = String;

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self, Self::Error> {
        self.serialize_map_called = true;
        Ok(self)
    }

    fn serialize_entry(self, _: &str, _: &Value) -> Result<Self::Ok, Self::Error> {
        Err("Mock error".into())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[test]
fn test_serialize_object_with_error() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let value_to_serialize = Value::Object(map);
    let serializer = MockSerializer {
        map_entries: vec![],
        serialize_map_called: false,
    };

    let result = value_to_serialize.serialize(serializer);
    assert_eq!(result, Err("Mock error".to_string()));
}

#[test]
fn test_serialize_object_success() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let value_to_serialize = Value::Object(map);
    
    let mock_serializer = MockSerializer {
        map_entries: vec![],
        serialize_map_called: false,
    };

    let result = value_to_serialize.serialize(mock_serializer);
    assert!(result.is_ok());
}

