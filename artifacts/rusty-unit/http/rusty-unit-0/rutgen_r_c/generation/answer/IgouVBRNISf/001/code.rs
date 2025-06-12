// Answer 0

#[test]
fn test_or_try_insert_with_vacant_entry_success() {
    struct TestHeaderMap {
        entries: Vec<(HeaderName, String)>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: Vec::new() }
        }

        fn try_entry(&mut self, key: &str) -> Option<Entry<String>> {
            if self.entries.iter().any(|(k, _)| k == &key.parse().unwrap()) {
                return Some(Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: 0,
                    index: self.entries.iter().position(|(k, _)| k == &key.parse().unwrap()).unwrap(),
                }));
            }
            Some(Entry::Vacant(VacantEntry {
                map: self,
                key: key.parse().unwrap(),
                hash: 0, // Simplified for this test
                probe: 0,
                danger: false,
            }))
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            self.entries.push((key, value));
        }
    }

    let mut map = TestHeaderMap::new();
    let res = map.try_entry("x-hello")
        .unwrap()
        .or_try_insert_with(|| "world".to_string())
        .unwrap();

    assert_eq!(res, "world");
}

#[test]
#[should_panic]
fn test_or_try_insert_with_occupied_entry_panic() {
    struct TestHeaderMap {
        entries: Vec<(HeaderName, String)>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: Vec::new() }
        }

        fn try_entry(&mut self, key: &str) -> Option<Entry<String>> {
            if self.entries.iter().any(|(k, _)| k == &key.parse().unwrap()) {
                return Some(Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: 0,
                    index: self.entries.iter().position(|(k, _)| k == &key.parse().unwrap()).unwrap(),
                }));
            }
            None
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            self.entries.push((key, value));
        }
    }

    let mut map = TestHeaderMap::new();
    map.insert("host".parse().unwrap(), "world".to_string());

    let _res = map.try_entry("host")
        .unwrap()
        .or_try_insert_with(|| unreachable!());
}

#[test]
fn test_or_try_insert_with_insert_value() {
    struct TestHeaderMap {
        entries: Vec<(HeaderName, String)>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap { entries: Vec::new() }
        }

        fn try_entry(&mut self, key: &str) -> Option<Entry<String>> {
            if self.entries.iter().any(|(k, _)| k == &key.parse().unwrap()) {
                return Some(Entry::Occupied(OccupiedEntry {
                    map: self,
                    probe: 0,
                    index: self.entries.iter().position(|(k, _)| k == &key.parse().unwrap()).unwrap(),
                }));
            }
            Some(Entry::Vacant(VacantEntry {
                map: self,
                key: key.parse().unwrap(),
                hash: 0,
                probe: 0,
                danger: false,
            }))
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            self.entries.push((key, value));
        }
    }

    let mut map = TestHeaderMap::new();
    let res = map.try_entry("new-key")
        .unwrap()
        .or_try_insert_with(|| "new-value".to_string())
        .unwrap();

    assert_eq!(res, "new-value");
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].1, "new-value");
}

