// Answer 0

#[test]
fn test_with_entries() {
    struct Entry {
        key: i32,
        value: i32,
    }

    struct TestMap {
        core: Vec<Entry>
    }

    impl TestMap {
        fn new() -> Self {
            Self { core: Vec::new() }
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Entry]),
        {
            f(&mut self.core);
        }
    }

    let mut map = TestMap::new();
    map.with_entries(|entries| {
        entries.push(Entry { key: 1, value: 100 });
        entries.push(Entry { key: 2, value: 200 });
    });

    assert_eq!(map.core.len(), 2);
    assert_eq!(map.core[0].key, 1);
    assert_eq!(map.core[0].value, 100);
    assert_eq!(map.core[1].key, 2);
    assert_eq!(map.core[1].value, 200);
}

