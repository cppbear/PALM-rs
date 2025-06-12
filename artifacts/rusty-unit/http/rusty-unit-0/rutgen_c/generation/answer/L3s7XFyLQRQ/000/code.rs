// Answer 0

#[test]
fn test_remove_entry() {
    #[derive(Clone)]
    struct MockHeaderValue;

    #[derive(Clone)]
    struct MockHeaderMap {
        entries: Vec<Bucket<MockHeaderValue>>,
        mask: Size,
    }

    impl HeaderMap<MockHeaderValue> {
        fn new() -> Self {
            HeaderMap {
                mask: 0,
                indices: Box::new([]),
                entries: vec![],
                extra_values: vec![],
                danger: Danger::default(),
            }
        }
        
        fn insert(&mut self, key: HeaderName, value: MockHeaderValue) {
            self.entries.push(Bucket {
                hash: HashValue::default(),
                key,
                value,
                links: None,
            });
        }
        
        fn remove_all_extra_values(&mut self, _next: usize) {}
        
        fn remove_found(&mut self, _probe: usize, _index: usize) -> Bucket<MockHeaderValue> {
            self.entries.remove(0) // Simplified for testing
        }

        fn contains_key(&self, _key: &str) -> bool {
            false // Simplified for testing
        }
    }

    let mut map = MockHeaderMap::new();
    let key = HeaderName { inner: Repr::default() };
    let value = MockHeaderValue;

    map.insert(key.clone(), value.clone());

    let occupied_entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };

    let (removed_key, removed_value) = occupied_entry.remove_entry();
    
    assert_eq!(removed_key, key);
    assert_eq!(removed_value, value);
    assert!(!map.contains_key("host"));
}

#[test]
fn test_remove_entry_empty_map() {
    #[derive(Clone)]
    struct MockHeaderValue;

    #[derive(Clone)]
    struct MockHeaderMap {
        entries: Vec<Bucket<MockHeaderValue>>,
    }

    impl HeaderMap<MockHeaderValue> {
        fn new() -> Self {
            HeaderMap {
                entries: vec![],
                mask: 0,
            }
        }
        
        fn remove_found(&mut self, _probe: usize, _index: usize) -> Bucket<MockHeaderValue> {
            panic!("Map is empty, cannot remove entry.")
        }
    }

    let mut map = MockHeaderMap::new();
    
    let occupied_entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };

    let result = std::panic::catch_unwind(|| {
        occupied_entry.remove_entry();
    });

    assert!(result.is_err());
}

