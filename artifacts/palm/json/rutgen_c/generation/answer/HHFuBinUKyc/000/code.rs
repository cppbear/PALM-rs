// Answer 0

#[test]
fn test_index_on_array() {
    struct ArrayIndex;
    impl Index for ArrayIndex {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Array(ref array) = value {
                array.get(0) // Testing with index 0
            } else {
                None
            }
        }
    }

    let value = Value::Array(vec![Value::String("zero".to_string()), Value::String("one".to_string())]);
    assert_eq!(value[ArrayIndex], Value::String("zero".to_string()));
    assert_eq!(value[1], Value::String("one".to_string()));
    assert_eq!(value[2], Value::Null); // Out of bounds
}

#[test]
fn test_index_on_object() {
    struct ObjectIndex<'a> {
        key: &'a str,
    }

    impl Index for ObjectIndex<'_> {
        fn index_into(&self, value: &Value) -> Option<&Value> {
            if let Value::Object(ref map) = value {
                map.map.get(self.key) // Retrieve the value by key
            } else {
                None
            }
        }
    }

    let mut object_map = Map::new(); // Assuming there's a new method for instantiation
    object_map.insert("key1".to_string(), Value::String("value1".to_string()));
    let value = Value::Object(object_map);

    assert_eq!(value[ObjectIndex { key: "key1" }], Value::String("value1".to_string()));
    assert_eq!(value[ObjectIndex { key: "invalid_key" }], Value::Null); // Key not found
}

#[test]
fn test_index_with_mixed_types() {
    let mixed_value = Value::Object(Map::new()); // Assume the map is empty for this test

    assert_eq!(mixed_value[0], Value::Null); // Index on object should return null
    assert_eq!(mixed_value["key"], Value::Null); // No keys should return null
}

