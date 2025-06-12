// Answer 0

#[test]
fn test_swap_remove_entry_valid() {
    struct TestIndexMapCore {
        entries: Vec<(i32, String)>,
    }

    impl TestIndexMapCore {
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            RefMut::new(&mut self.entries)
        }

        fn swap_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.entries.len() {
                Some(self.entries.swap_remove(index))
            } else {
                None
            }
        }
    }

    // Initialize a test index map core with some entries.
    let mut map = TestIndexMapCore {
        entries: vec![(1, "one".into()), (2, "two".into()), (3, "three".into())],
    };

    let entry = IndexedEntry::new(&mut map, 1);
    let (key, value) = entry.swap_remove_entry();

    assert_eq!(key, 2);
    assert_eq!(value, "two");
    assert_eq!(map.entries.len(), 2);
    assert!(!map.entries.iter().any(|&(k, _)| k == 2));
}

#[should_panic]
fn test_swap_remove_entry_invalid_index() {
    struct TestIndexMapCore {
        entries: Vec<(i32, String)>,
    }

    impl TestIndexMapCore {
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            RefMut::new(&mut self.entries)
        }

        fn swap_remove_index(&mut self, _index: usize) -> Option<(i32, String)> {
            // Returning None simulating an invalid index access
            None
        }
    }

    // Initialize a test index map core with no entries.
    let mut map = TestIndexMapCore {
        entries: vec![],
    };

    let entry = IndexedEntry::new(&mut map, 0);
    entry.swap_remove_entry(); // This should panic due to invalid index.
}

