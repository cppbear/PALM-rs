// Answer 0

#[test]
fn test_get_valid_entry() {
    #[derive(Debug)]
    struct TestHeaderMap {
        entries: Vec<Bucket<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            self.entries.push(Bucket {
                hash: 0,
                key,
                value,
                links: None,
            });
        }
    }

    let mut map = TestHeaderMap::new();
    let key = HeaderName::from_static("host");
    map.insert(key.clone(), "hello.world".to_string());

    let occupied_entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };

    assert_eq!(occupied_entry.get(), &"hello.world".to_string());
}

#[should_panic]
#[test]
fn test_get_empty_entry() {
    #[derive(Debug)]
    struct TestHeaderMap {
        entries: Vec<Bucket<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
    }

    let mut map = TestHeaderMap::new();

    // Attempt to create an occupied entry without inserting any entries
    let occupied_entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };

    // This should panic as there are no entries.
    occupied_entry.get();
} 

#[test]
fn test_get_after_append() {
    #[derive(Debug)]
    struct TestHeaderMap {
        entries: Vec<Bucket<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn insert(&mut self, key: HeaderName, value: String) {
            self.entries.push(Bucket {
                hash: 0,
                key,
                value,
                links: None,
            });
        }
    }

    let mut map = TestHeaderMap::new();
    let key = HeaderName::from_static("host");
    map.insert(key.clone(), "hello.world".to_string());

    let mut occupied_entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };

    assert_eq!(occupied_entry.get(), &"hello.world".to_string());

    occupied_entry.append("hello.earth".to_string());

    // Check that get still returns the first value
    assert_eq!(occupied_entry.get(), &"hello.world".to_string());
}

