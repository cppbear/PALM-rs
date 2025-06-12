// Answer 0

#[test]
fn test_get_existing_key() {
    struct TestMap {
        core: IndexMapCore<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            let core = IndexMapCore {
                indices: Indices::new(),
                entries: Entries::new(),
            };
            TestMap { core }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.core.entries.push(Bucket {
                hash: HashValue::default(),
                key,
                value,
            });
            // Assume the necessary changes to indices are made here
        }

        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.core.entries
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            // Mock implementation: Just return the index if key matches
            self.as_entries().iter().position(|entry| entry.key == key)
        }

        fn get<Q>(&self, key: &Q) -> Option<&i32>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some(&entry.value)
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.insert("test".to_string(), 42);
    assert_eq!(map.get(&"test".to_string()), Some(&42));
}

#[test]
fn test_get_non_existing_key() {
    struct TestMap {
        core: IndexMapCore<String, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            let core = IndexMapCore {
                indices: Indices::new(),
                entries: Entries::new(),
            };
            TestMap { core }
        }

        fn insert(&mut self, key: String, value: i32) {
            self.core.entries.push(Bucket {
                hash: HashValue::default(),
                key,
                value,
            });
            // Assume the necessary changes to indices are made here
        }

        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.core.entries
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.as_entries().iter().position(|entry| entry.key == key)
        }

        fn get<Q>(&self, key: &Q) -> Option<&i32>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some(&entry.value)
            } else {
                None
            }
        }
    }

    let map = TestMap::new();
    assert_eq!(map.get(&"non_existing_key".to_string()), None);
}

