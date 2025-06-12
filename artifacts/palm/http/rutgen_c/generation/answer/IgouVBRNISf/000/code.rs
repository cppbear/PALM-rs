// Answer 0

#[test]
fn test_or_try_insert_with_vacant_entry() {
    struct TestHeaderMap {
        entries: Vec<Option<(HeaderName, String)>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: vec![None; 16] }
        }

        fn try_entry(&mut self, key: &str) -> Option<Entry<String>> {
            let index = key.len() % self.entries.len(); // Simple hash for testing
            if let Some((_, _)) = &self.entries[index] {
                Some(Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: index,
                    index: index,
                }))
            } else {
                Some(Entry::Vacant(VacantEntry {
                    map: self,
                    key: HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    hash: 0,
                    probe: index,
                    danger: false,
                }))
            }
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            let index = key.as_str().len() % self.entries.len(); // Simplified insert
            self.entries[index] = Some((key, value));
        }
    }

    let mut map = TestHeaderMap::new();
    let res = map.try_entry("x-hello").unwrap()
        .or_try_insert_with(|| String::from("world")).unwrap();

    assert_eq!(res, "world");
}

#[test]
fn test_or_try_insert_with_occupied_entry() {
    struct TestHeaderMap {
        entries: Vec<Option<(HeaderName, String)>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: vec![None; 16] }
        }

        fn try_insert(&mut self, key: HeaderName, value: String) -> Result<(), MaxSizeReached> {
            // Simple insert for testing
            self.insert(key, value);
            Ok(())
        }

        fn try_entry(&mut self, key: &str) -> Option<Entry<String>> {
            let index = key.len() % self.entries.len(); // Simple hash for testing
            if let Some((_, _)) = &self.entries[index] {
                Some(Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: index,
                    index: index,
                }))
            } else {
                None // No entry
            }
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            let index = key.as_str().len() % self.entries.len(); // Simplified insert
            self.entries[index] = Some((key, value));
        }
    }

    let mut map = TestHeaderMap::new();
    map.try_insert(HeaderName::from_bytes("host".as_bytes()).unwrap(), String::from("world")).unwrap();
    
    let res = map.try_entry("host").unwrap()
        .or_try_insert_with(|| String::from("unreachable!")).unwrap();

    assert_eq!(res, "world");
}

