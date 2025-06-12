// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            Self {
                entries: vec![
                    Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
                    Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
                    Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() },
                ],
            }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn get_index(&self, index: usize) -> Option<(&i32, &String)> {
            self.as_entries().get(index).map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let index_map = TestIndexMap::new();

    assert_eq!(index_map.get_index(0), Some((&1, &"one".to_string())));
    assert_eq!(index_map.get_index(1), Some((&2, &"two".to_string())));
    assert_eq!(index_map.get_index(2), Some((&3, &"three".to_string())));
}

#[test]
#[should_panic]
fn test_get_index_out_of_bounds() {
    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestIndexMap {
        fn new() -> Self {
            Self {
                entries: vec![
                    Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
                ],
            }
        }

        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn get_index(&self, index: usize) -> Option<(&i32, &String)> {
            self.as_entries().get(index).map(|bucket| (&bucket.key, &bucket.value))
        }
    }

    let index_map = TestIndexMap::new();

    // Accessing index 1 should panic as there is only one element at index 0.
    index_map.get_index(1).unwrap();
}

