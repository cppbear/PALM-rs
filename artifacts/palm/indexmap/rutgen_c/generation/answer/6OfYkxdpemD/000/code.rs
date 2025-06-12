// Answer 0

#[test]
fn test_indexmap_new() {
    // Define a minimal version of IndexMap with Default hasher for testing
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(), // Assuming there is a new() method
                    entries: Entries::new(),  // Assuming there is a new() method
                },
            }
        }
    }

    let map = TestMap::new();
    assert_eq!(map.core.entries.as_entries().len(), 0);
}

#[test]
fn test_indexmap_with_capacity() {
    // Define a minimal version of IndexMap with Default hasher for testing
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn with_capacity(n: usize) -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(), // Assuming there is a new() method
                    entries: Entries::with_capacity(n), // Assuming there is a with_capacity() method
                },
            }
        }
    }

    let map = TestMap::with_capacity(10);
    assert_eq!(map.core.entries.as_entries().capacity(), 10);
}

