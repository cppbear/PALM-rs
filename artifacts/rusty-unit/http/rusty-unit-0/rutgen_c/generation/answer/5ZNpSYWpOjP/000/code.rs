// Answer 0

#[test]
fn test_or_try_insert_occupied() {
    struct TestMap {
        entries: Vec<(HeaderName, u32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn insert(&mut self, key: HeaderName, value: u32) {
            self.entries.push((key, value));
        }

        fn entry(&mut self, key: &str) -> Entry<u32> {
            let header_name = HeaderName::from_str(key).unwrap();
            if let Some(pos) = self.entries.iter().position(|(k, _)| k == &header_name) {
                Entry::Occupied(OccupiedEntry {
                    map: self,
                    index: pos,
                    probe: 0, // Simplified for the test
                })
            } else {
                Entry::Vacant(VacantEntry {
                    map: self,
                    key: header_name,
                    hash: 0, // Dummy value
                    probe: 0,
                    danger: false,
                })
            }
        }
    }

    let mut map = TestMap::new();
    map.insert(HeaderName::from_str("x-hello").unwrap(), 1);

    let counter = map.entry("x-hello").or_try_insert(0).unwrap();
    *counter += 1;

    assert_eq!(map.entries[0].1, 2);
}

#[test]
fn test_or_try_insert_vacant() {
    struct TestMap {
        entries: Vec<(HeaderName, u32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn entry(&mut self, key: &str) -> Entry<u32> {
            let header_name = HeaderName::from_str(key).unwrap();
            if let Some(pos) = self.entries.iter().position(|(k, _)| k == &header_name) {
                Entry::Occupied(OccupiedEntry {
                    map: self,
                    index: pos,
                    probe: 0, // Simplified for the test
                })
            } else {
                Entry::Vacant(VacantEntry {
                    map: self,
                    key: header_name,
                    hash: 0, // Dummy value
                    probe: 0,
                    danger: false,
                })
            }
        }

        fn try_insert_phase_two(&mut self, _key: HeaderName, value: u32, _hash: usize, _probe: usize, _danger: bool) -> Result<usize, MaxSizeReached> {
            self.entries.push((_key, value));
            Ok(self.entries.len() - 1)
        }
    }

    let mut map = TestMap::new();

    let counter = map.entry("x-world").or_try_insert(0).unwrap();
    *counter += 1;

    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].1, 1);
}

#[test]
#[should_panic]
fn test_or_try_insert_max_size_reached() {
    struct TestMap {
        entries: Vec<(HeaderName, u32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn entry(&mut self, key: &str) -> Entry<u32> {
            let header_name = HeaderName::from_str(key).unwrap();
            if let Some(pos) = self.entries.iter().position(|(k, _)| k == &header_name) {
                Entry::Occupied(OccupiedEntry {
                    map: self,
                    index: pos,
                    probe: 0, // Simplified for the test
                })
            } else {
                Entry::Vacant(VacantEntry {
                    map: self,
                    key: header_name,
                    hash: 0, // Dummy value
                    probe: 0,
                    danger: false,
                })
            }
        }

        fn try_insert_phase_two(&mut self, key: HeaderName, _value: u32, _hash: usize, _probe: usize, _danger: bool) -> Result<usize, MaxSizeReached> {
            if self.entries.len() >= 1 { // Example limit
                Err(MaxSizeReached { _priv: () })
            } else {
                self.entries.push((key, 0));
                Ok(self.entries.len() - 1)
            }
        }
    }

    let mut map = TestMap::new();
    map.entry("x-world").or_try_insert(0).unwrap();
    map.entry("x-world").or_try_insert(0).unwrap(); // This triggers the panic
}

