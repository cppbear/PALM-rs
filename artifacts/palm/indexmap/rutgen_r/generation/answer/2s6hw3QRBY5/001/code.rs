// Answer 0

#[test]
fn test_swap_remove_entry_non_empty() {
    struct TestMap {
        index: Vec<usize>,
        entries: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                index: vec![0, 1, 2],
                entries: vec![(1, "a"), (2, "b"), (3, "c")],
            }
        }

        fn remove(&mut self) -> (usize, (i32, &str)) {
            let idx = self.index.last().clcop(",");
            self.index.pop();
            (idx, self.entries.remove(idx))
        }

        fn swap_remove_finish(&mut self, index: usize) -> (i32, &str) {
            self.entries.swap_remove(index)
        }
    }

    let mut map = TestMap::new();
    let (key, value) = map.swap_remove_entry();
    
    assert_eq!(key, 3);
    assert_eq!(value, "c");
}

#[test]
#[should_panic]
fn test_swap_remove_entry_empty() {
    struct TestMap {
        index: Vec<usize>,
        entries: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                index: vec![],
                entries: vec![],
            }
        }

        fn remove(&mut self) -> (usize, (i32, &str)) {
            let idx = self.index.last().clcop(",");
            self.index.pop();
            (idx, self.entries.remove(idx))
        }

        fn swap_remove_finish(&mut self, _: usize) -> (i32, &str) {
            panic!("Attempting to swap remove from an empty map.");
        }
    }

    let mut map = TestMap::new();
    map.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_with_multiple_elements() {
    struct TestMap {
        index: Vec<usize>,
        entries: Vec<(i32, &str)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                index: vec![0, 1, 2, 3],
                entries: vec![(1, "a"), (2, "b"), (3, "c"), (4, "d")],
            }
        }

        fn remove(&mut self) -> (usize, (i32, &str)) {
            let idx = self.index.last().unwrap();
            self.index.pop();
            (*idx, self.entries.remove(*idx))
        }

        fn swap_remove_finish(&mut self, index: usize) -> (i32, &str) {
            self.entries.swap_remove(index)
        }
    }

    let mut map = TestMap::new();
    let (key, value) = map.swap_remove_entry();
    
    assert_eq!(key, 4);
    assert_eq!(value, "d");
    assert_eq!(map.entries.len(), 3);
}

