// Answer 0

#[test]
fn test_get_existing_key() {
    struct TestIndexMap {
        entries: Vec<Bucket<String, i32>>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                entries: vec![
                    Bucket { hash: HashValue(1), key: "key1".to_string(), value: 10 },
                    Bucket { hash: HashValue(2), key: "key2".to_string(), value: 20 },
                ],
            }
        }

        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.entries
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.entries.iter().position(|b| &b.key == key)
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

    let index_map = TestIndexMap::new();
    let result = index_map.get(&"key1");
    assert_eq!(result, Some(&10));
}

#[test]
fn test_get_non_existing_key() {
    struct TestIndexMap {
        entries: Vec<Bucket<String, i32>>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                entries: vec![
                    Bucket { hash: HashValue(1), key: "key1".to_string(), value: 10 },
                    Bucket { hash: HashValue(2), key: "key2".to_string(), value: 20 },
                ],
            }
        }

        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.entries
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.entries.iter().position(|b| &b.key == key)
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

    let index_map = TestIndexMap::new();
    let result = index_map.get(&"key3");
    assert_eq!(result, None);
}

