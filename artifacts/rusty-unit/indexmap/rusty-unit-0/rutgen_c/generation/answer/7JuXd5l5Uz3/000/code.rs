// Answer 0

#[test]
fn test_len_empty_map() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn len(&self) -> usize {
            self.core.len()
        }
    }

    let map = TestMap::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_non_empty_map() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new_with_entries(entries: Vec<(i32, i32)>) -> Self {
            let mut core = IndexMapCore::new();
            for (k, v) in entries {
                core.entries.push((k, v)); // Simplified assumption
            }
            core.indices = Indices::with_capacity(core.entries.len()); // Simplified assumption
            TestMap { core }
        }

        fn len(&self) -> usize {
            self.core.len()
        }
    }

    let map = TestMap::new_with_entries(vec![(1, 10), (2, 20)]);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_len_after_clear() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new_with_entries(entries: Vec<(i32, i32)>) -> Self {
            let mut core = IndexMapCore::new();
            for (k, v) in entries {
                core.entries.push((k, v)); // Simplified assumption
            }
            core.indices = Indices::with_capacity(core.entries.len()); // Simplified assumption
            TestMap { core }
        }

        fn clear(&mut self) {
            self.core.clear(); // Assumed to clear indices and entries
        }

        fn len(&self) -> usize {
            self.core.len()
        }
    }

    let mut map = TestMap::new_with_entries(vec![(1, 10), (2, 20)]);
    assert_eq!(map.len(), 2);
    map.clear();
    assert_eq!(map.len(), 0);
}

