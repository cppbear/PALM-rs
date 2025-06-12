// Answer 0

#[test]
fn test_to_vec_pretty_with_non_serializable_key() {
    use serde::Serialize;
    use serde_json::Error;

    struct NonSerializableKey {
        value: i32,
    }
    
    struct Wrapper {
        map: std::collections::HashMap<NonSerializableKey, String>,
    }

    let mut map = std::collections::HashMap::new();
    map.insert(NonSerializableKey { value: 42 }, "value".to_string());
    
    let wrapper = Wrapper { map };

    let result: Result<Vec<u8>, Error> = serde_json::to_vec_pretty(&wrapper);
    assert!(result.is_err());
}

