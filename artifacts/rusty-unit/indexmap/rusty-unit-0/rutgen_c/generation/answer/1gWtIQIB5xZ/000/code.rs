// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestMap {
        entries: Vec<Bucket<i32, &str>>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![
                    Bucket { hash: HashValue::default(), key: 1, value: "one" },
                    Bucket { hash: HashValue::default(), key: 2, value: "two" },
                    Bucket { hash: HashValue::default(), key: 3, value: "three" },
                ],
            }
        }

        fn as_entries(&self) -> &Vec<Bucket<i32, &str>> {
            &self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index(&self, index: usize) -> Option<(&i32, &str)> {
            self.as_entries().get(index).map(|bucket| (&bucket.key, bucket.value))
        }
    }

    let map = TestMap::new();
    assert_eq!(map.get_index(0), Some((&1, "one")));
    assert_eq!(map.get_index(1), Some((&2, "two")));
    assert_eq!(map.get_index(2), Some((&3, "three")));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestMap {
        entries: Vec<Bucket<i32, &str>>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![
                    Bucket { hash: HashValue::default(), key: 1, value: "one" },
                    Bucket { hash: HashValue::default(), key: 2, value: "two" },
                ],
            }
        }

        fn as_entries(&self) -> &Vec<Bucket<i32, &str>> {
            &self.entries
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn get_index(&self, index: usize) -> Option<(&i32, &str)> {
            self.as_entries().get(index).map(|bucket| (&bucket.key, bucket.value))
        }
    }

    let map = TestMap::new();
    assert_eq!(map.get_index(2), None); // Out of bounds
}

