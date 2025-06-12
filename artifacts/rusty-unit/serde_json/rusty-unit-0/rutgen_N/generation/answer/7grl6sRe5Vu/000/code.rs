// Answer 0

#[derive(Debug)]
struct SerializeMap {
    map: std::collections::HashMap<String, serde_json::Value>,
}

// Implement a constructor for SerializeMap
impl SerializeMap {
    fn new(map: std::collections::HashMap<String, serde_json::Value>) -> Self {
        SerializeMap { map }
    }
}

// Example implementation of Value to mimic serde_json::Value
#[derive(Debug)]
enum Value {
    Object(std::collections::HashMap<String, serde_json::Value>),
}

impl SerializeMap {
    fn end(self) -> Result<Value, &'static str> {
        match self {
            SerializeMap { map, .. } => Ok(Value::Object(map)),
        }
    }
}

#[test]
fn test_end_with_empty_map() {
    let map: std::collections::HashMap<String, serde_json::Value> = std::collections::HashMap::new();
    let serialize_map = SerializeMap::new(map);
    let result = serialize_map.end().unwrap();
    match result {
        Value::Object(map) => assert_eq!(map.len(), 0),
    }
}

#[test]
fn test_end_with_non_empty_map() {
    let mut map = std::collections::HashMap::new();
    map.insert("key".to_string(), serde_json::Value::String("value".to_string()));
    let serialize_map = SerializeMap::new(map);
    let result = serialize_map.end().unwrap();
    match result {
        Value::Object(map) => {
            assert_eq!(map.len(), 1);
            assert_eq!(map.get("key").unwrap(), &serde_json::Value::String("value".to_string()));
        }
    }
}

