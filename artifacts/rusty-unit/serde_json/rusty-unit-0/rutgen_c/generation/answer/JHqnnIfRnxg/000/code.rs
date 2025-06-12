// Answer 0

#[test]
fn test_vacant_entry_insert() {
    use std::collections::BTreeMap;
    use serde_json::value::Value;
    
    struct Map {
        inner: BTreeMap<String, Value>,
    }
    
    impl Map {
        fn new() -> Self {
            Self {
                inner: BTreeMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry {
            if self.inner.contains_key(key) {
                Entry::Occupied(OccupiedEntry {})
            } else {
                let vacant_entry = self.inner.entry(key.to_string()).or_insert(Value::Null);
                Entry::Vacant(VacantEntry { vacant: vacant_entry })
            }
        }
    }
    
    struct OccupiedEntry {}

    pub enum Entry<'a> {
        Vacant(VacantEntry<'a>),
        Occupied(OccupiedEntry),
    }

    impl<'a> VacantEntry<'a> {
        pub fn key(&self) -> &String {
            self.vacant
        }
    }
    
    let mut map = Map::new();
    
    match map.entry("serde") {
        Entry::Vacant(vacant) => {
            let value = Value::String("hoho".to_string());
            let inserted_value = vacant.insert(value);
            assert_eq!(inserted_value, &Value::String("hoho".to_string()));
        }
        Entry::Occupied(_) => panic!("Entry should be vacant"),
    }
}

#[test]
fn test_vacant_entry_insert_with_existing_key() {
    use std::collections::BTreeMap;
    use serde_json::value::Value;

    struct Map {
        inner: BTreeMap<String, Value>,
    }
    
    impl Map {
        fn new() -> Self {
            Self {
                inner: BTreeMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry {
            if self.inner.contains_key(key) {
                Entry::Occupied(OccupiedEntry {})
            } else {
                let vacant_entry = self.inner.entry(key.to_string()).or_insert(Value::Null);
                Entry::Vacant(VacantEntry { vacant: vacant_entry })
            }
        }
    }

    struct OccupiedEntry {}

    pub enum Entry<'a> {
        Vacant(VacantEntry<'a>),
        Occupied(OccupiedEntry),
    }

    let mut map = Map::new();
    map.inner.insert("serde".to_string(), Value::String("initial".to_string()));

    match map.entry("serde") {
        Entry::Vacant(_) => panic!("Entry should be occupied"),
        Entry::Occupied(_) => { /* do nothing */ }
    }
}

