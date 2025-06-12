// Answer 0

#[test]
fn test_and_modify_on_vacant_entry() {
    struct TestMap {
        inner: MapImpl<String, Value>,
    }

    impl TestMap {
        fn entry(&mut self, key: &str) -> Entry {
            if self.inner.contains_key(key) {
                Entry::Occupied(OccupiedEntry {
                    occupied: self.inner.get_mut(key).unwrap(),
                })
            } else {
                let vacant_entry = VacantEntry {
                    vacant: self.inner.entry(key.to_string()).or_insert(Value::Null),
                };
                Entry::Vacant(vacant_entry)
            }
        }

        fn new() -> Self {
            Self {
                inner: MapImpl::new(),
            }
        }
    }

    let mut map = TestMap::new();
    let entry = map.entry("key");
    
    // Ensure it returns a Vacant entry
    match entry {
        Entry::Vacant(_) => {}
        _ => panic!("Expected Vacant entry"),
    }

    // Since it did not panic, we can check if it inserted the default value (Value::Null).
    assert_eq!(map.inner.len(), 1);
    assert_eq!(map.inner.get("key"), Some(&Value::Null));
}

