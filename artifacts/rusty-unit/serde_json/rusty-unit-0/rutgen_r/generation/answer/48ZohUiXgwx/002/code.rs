// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use serde_json::json;
    use serde_json::Map;
    use serde_json::Value;

    struct Entry<'a> {
        map: &'a mut Map<String, Value>,
        key: String,
    }

    impl<'a> Entry<'a> {
        fn vacant(map: &'a mut Map<String, Value>, key: String) -> Self {
            Entry { map, key }
        }

        fn insert(self, default: Value) -> &'a mut Value {
            self.map.insert(self.key, default);
            self.map.get_mut(&self.key).unwrap()
        }
    }

    let mut map = Map::new();
    let entry = Entry::vacant(&mut map, "test_key".to_string());
    
    assert_eq!(*entry.insert(json!(42)), json!(42));
    assert_eq!(map.get("test_key").unwrap(), &json!(42));
}

#[test]
fn test_or_insert_occupied_entry() {
    use serde_json::json;
    use serde_json::Map;
    use serde_json::Value;

    struct Entry<'a> {
        map: &'a mut Map<String, Value>,
        key: String,
    }

    impl<'a> Entry<'a> {
        fn occupied(map: &'a mut Map<String, Value>, key: String) -> Self {
            Entry { map, key }
        }

        fn insert(self, default: Value) -> &'a mut Value {
            self.map.insert(self.key.clone(), default);
            self.map.get_mut(&self.key).unwrap()
        }

        fn into_mut(self) -> &'a mut Value {
            self.map.get_mut(&self.key).unwrap()
        }
    }

    let mut map = Map::new();
    let key = "existing_key".to_string();
    map.insert(key.clone(), json!(100));

    let entry = Entry::occupied(&mut map, key);
    let existing_value = entry.into_mut();

    assert_eq!(*existing_value, json!(100));
}

