// Answer 0

#[test]
fn test_to_vec_pretty_with_non_serializable_value() {
    struct NonSerializable;
    
    // Attempting to serialize a struct that does not implement Serialize
    let value = NonSerializable;
    
    let result: Result<Vec<u8>> = to_vec_pretty(&value);
    assert!(result.is_err());  // Must return an error due to non-serializability
}

#[test]
fn test_to_vec_pretty_with_map_with_non_string_keys() {
    use std::collections::HashMap;

    struct NonStringKeyStruct;

    impl Serialize for NonStringKeyStruct {
        // Implementing Serialize for a struct that contains a HashMap with non-string keys
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut map = HashMap::new();
            // Using an integer as a key which is not allowed
            map.insert(1, "value");
            serializer.serialize_map(Some(map.len()))?.end()
        }
    }

    let value = NonStringKeyStruct;
    
    let result: Result<Vec<u8>> = to_vec_pretty(&value);
    assert!(result.is_err());  // Must return an error due to non-string keys in the map
}

