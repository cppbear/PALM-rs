// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use serde_json::{json, Map, Value, MapEntry};

    struct VacantEntry<'a> {
        map: &'a mut Map<String, Value>,
        key: String,
    }

    impl<'a> VacantEntry<'a> {
        fn insert(self, default: Value) -> &'a mut Value {
            self.map.insert(self.key, default);
            self.map.get_mut(&self.key).unwrap()
        }
    }

    enum Entry<'a> {
        Vacant(VacantEntry<'a>),
        Occupied(&'a mut Value),
    }

    impl<'a> Entry<'a> {
        fn or_insert_with<F>(self, default: F) -> &'a mut Value
        where
            F: FnOnce() -> Value,
        {
            match self {
                Entry::Vacant(entry) => entry.insert(default()),
                Entry::Occupied(entry) => entry,
            }
        }
    }

    let mut map = Map::new();
    let key = "serde".to_string();
    let entry = Entry::Vacant(VacantEntry { map: &mut map, key });

    let value = entry.or_insert_with(|| json!("hoho"));
    assert_eq!(value, &mut json!("hoho"));
    assert_eq!(map["serde"], "hoho".to_owned());
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use serde_json::{json, Map, Value};

    let mut map = Map::new();
    map.insert("serde".to_string(), json!("existing_value"));

    let key = "serde".to_string();
    let mut value_ref = map.get_mut(&key).unwrap();

    enum Entry<'a> {
        Vacant(VacantEntry<'a>),
        Occupied(&'a mut Value),
    }

    impl<'a> Entry<'a> {
        fn or_insert_with<F>(self, default: F) -> &'a mut Value
        where
            F: FnOnce() -> Value,
        {
            match self {
                Entry::Vacant(entry) => entry.insert(default()),
                Entry::Occupied(entry) => entry,
            }
        }
    }

    let entry = Entry::Occupied(&mut value_ref);
    let value = entry.or_insert_with(|| json!("hoho"));
    assert_eq!(value, &mut json!("existing_value"));
    assert_eq!(map["serde"], "existing_value");
}

