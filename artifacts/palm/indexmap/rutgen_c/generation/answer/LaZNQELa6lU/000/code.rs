// Answer 0

#[test]
fn test_iter_mut() {
    struct TestMap {
        core: IndexMapCore<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                },
            }
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            // Assume we would have some method to return mutable access to the entries
            // This is a simplification for the sake of the test
            &mut self.core.entries.as_mut()
        }

        fn iter_mut(&mut self) -> IterMut<i32, String> {
            IterMut::new(self.as_entries_mut())
        }
    }

    let mut test_map = TestMap::new();
    // Assume we have added some key-value pairs here:
    // For example, test_map.core.entries.push(Bucket { hash: HashValue::new(), key: 1, value: "one".to_string() });
    // For simplicity, this example focuses on the structure; insertion logic should be added.

    let mut iter = test_map.iter_mut();
    while let Some(entry) = iter.next() {
        // Perform assertions based on the entries
        // For example: assert_eq!(entry.key, expected_key);
    }
}

#[test]
fn test_iter_mut_empty() {
    struct TestMap {
        core: IndexMapCore<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                },
            }
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.core.entries.as_mut()
        }

        fn iter_mut(&mut self) -> IterMut<i32, String> {
            IterMut::new(self.as_entries_mut())
        }
    }

    let mut test_map = TestMap::new(); // Empty map

    let mut iter = test_map.iter_mut();
    assert!(iter.next().is_none()); // Ensure the iterator is empty
}

