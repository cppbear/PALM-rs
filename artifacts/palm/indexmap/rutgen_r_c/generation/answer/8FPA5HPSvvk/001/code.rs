// Answer 0

#[test]
fn test_last_mut_non_empty() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn push(&mut self, key: i32, value: String) {
            let bucket = Bucket {
                hash: HashValue::default(), // Assuming a default value for simplicity
                key,
                value,
            };
            self.entries.push(bucket);
        }

        fn as_entries_mut(&mut self) -> &mut Vec<Bucket<i32, String>> {
            &mut self.entries
        }

        fn last_mut(&mut self) -> Option<(&i32, &mut String)> {
            self.as_entries_mut().last_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut index_map = TestIndexMap::new();
    index_map.push(1, "value1".to_string());
    index_map.push(2, "value2".to_string());

    let last = index_map.last_mut();
    assert!(last.is_some());
    if let Some((key, value)) = last {
        assert_eq!(*key, 2);
        assert_eq!(*value, "value2");
        *value = "updated_value".to_string();
    }

    assert_eq!(index_map.entries.last().unwrap().value, "updated_value");
}

#[test]
fn test_last_mut_empty() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn as_entries_mut(&mut self) -> &mut Vec<Bucket<i32, String>> {
            &mut self.entries
        }

        fn last_mut(&mut self) -> Option<(&i32, &mut String)> {
            self.as_entries_mut().last_mut().map(|bucket| (&bucket.key, &mut bucket.value))
        }
    }

    let mut index_map = TestIndexMap::new();

    let last = index_map.last_mut();
    assert!(last.is_none());
}

