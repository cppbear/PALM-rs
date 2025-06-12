// Answer 0

#[test]
fn test_serialize_value_map() {
    use serde::Serialize;
    use std::collections::HashMap;

    struct SerializeMap {
        map: HashMap<String, String>,
        next_key: Option<String>,
    }

    impl SerializeMap {
        fn new() -> Self {
            SerializeMap {
                map: HashMap::new(),
                next_key: Some("key".to_string()),
            }
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            match self {
                SerializeMap { map, next_key } => {
                    let key = next_key.take();
                    let key = key.expect("serialize_value called before serialize_key");
                    let serialized_value = serde_json::to_string(value).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "serialization error"))?;
                    map.insert(key, serialized_value);
                    Ok(())
                }
            }
        }
    }

    let mut serialize_map = SerializeMap::new();
    let value = "test_value";

    let result = serialize_map.serialize_value(&value);
    assert!(result.is_ok());
    assert_eq!(serialize_map.map.get("key"), Some(&r#""test_value""#.to_string()));
}

#[test]
#[should_panic(expected = "serialize_value called before serialize_key")]
fn test_serialize_value_no_key() {
    use serde::Serialize;
    use std::collections::HashMap;

    struct SerializeMap {
        map: HashMap<String, String>,
        next_key: Option<String>,
    }

    impl SerializeMap {
        fn new() -> Self {
            SerializeMap {
                map: HashMap::new(),
                next_key: None,
            }
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            match self {
                SerializeMap { map, next_key } => {
                    let key = next_key.take();
                    let key = key.expect("serialize_value called before serialize_key");
                    let serialized_value = serde_json::to_string(value).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "serialization error"))?;
                    map.insert(key, serialized_value);
                    Ok(())
                }
            }
        }
    }

    let mut serialize_map = SerializeMap::new();
    let value = "test_value";

    serialize_map.serialize_value(&value).unwrap();
}

