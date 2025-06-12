// Answer 0

#[test]
fn test_values_mut() {
    struct TestHasher;
    struct TestIndexMap {
        core: IndexMapCore<i32, String>,
        hash_builder: TestHasher,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                core: IndexMapCore {
                    indices: Default::default(),
                    entries: Default::default(),
                },
                hash_builder: TestHasher,
            }
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.core.entries
        }
    }

    let mut index_map = TestIndexMap::new();
    index_map.core.entries.push(Bucket {
        hash: HashValue::default(),
        key: 1,
        value: "one".to_string(),
    });
    index_map.core.entries.push(Bucket {
        hash: HashValue::default(),
        key: 2,
        value: "two".to_string(),
    });

    let values_mut_iter = index_map.values_mut();
    let mut values: Vec<&mut String> = values_mut_iter.iter.collect();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], &mut "one".to_string());
    assert_eq!(values[1], &mut "two".to_string());

    values[0].push_str(" updated");
    assert_eq!(index_map.core.entries[0].value, "one updated");
}

#[test]
#[should_panic]
fn test_values_mut_empty() {
    struct TestHasher;
    struct TestIndexMap {
        core: IndexMapCore<i32, String>,
        hash_builder: TestHasher,
    }

    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap {
                core: IndexMapCore {
                    indices: Default::default(),
                    entries: Default::default(),
                },
                hash_builder: TestHasher,
            }
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.core.entries
        }
    }

    let mut index_map = TestIndexMap::new();
    let _ = index_map.values_mut(); // Should not panic, but will trigger panic if asserting empty state later
    assert!(index_map.core.entries.is_empty());
}

