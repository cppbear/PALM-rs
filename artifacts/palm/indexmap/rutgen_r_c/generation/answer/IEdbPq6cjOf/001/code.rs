// Answer 0

#[test]
fn test_insert_value() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            // For the sake of this test, the logic isn't implemented deeply, but a reference is returned.
            RefMut {
                indices: &mut (),
                entries: &mut self.entries,
            }
        }
    }

    let mut test_map = TestMap {
        entries: vec![(1, "old_value".to_string())],
    };

    let mut indexed_entry = IndexedEntry::new(&mut test_map, 0);
    let old_value = indexed_entry.insert("new_value".to_string());

    assert_eq!(old_value, "old_value");
    assert_eq!(indexed_entry.get(), &"new_value".to_string());
}

#[test]
fn test_insert_empty_old_value() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            RefMut {
                indices: &mut (),
                entries: &mut self.entries,
            }
        }
    }

    let mut test_map = TestMap {
        entries: vec![(1, "".to_string())],
    };

    let mut indexed_entry = IndexedEntry::new(&mut test_map, 0);
    let old_value = indexed_entry.insert("new_value".to_string());

    assert_eq!(old_value, "");
    assert_eq!(indexed_entry.get(), &"new_value".to_string());
}

#[test]
fn test_insert_same_value() {
    struct TestMap {
        entries: Vec<(i32, String)>,
    }

    impl TestMap {
        fn borrow_mut(&mut self) -> RefMut<i32, String> {
            RefMut {
                indices: &mut (),
                entries: &mut self.entries,
            }
        }
    }

    let mut test_map = TestMap {
        entries: vec![(1, "same_value".to_string())],
    };

    let mut indexed_entry = IndexedEntry::new(&mut test_map, 0);
    let old_value = indexed_entry.insert("same_value".to_string());

    assert_eq!(old_value, "same_value");
    assert_eq!(indexed_entry.get(), &"same_value".to_string());
}

