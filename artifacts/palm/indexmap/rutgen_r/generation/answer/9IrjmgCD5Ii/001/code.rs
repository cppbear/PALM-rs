// Answer 0

#[test]
fn test_shift_remove_entry_valid_case() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.data.len() {
                let item = self.data.remove(index);
                Some(item)
            } else {
                None
            }
        }
    }

    struct Entry<K, V> {
        map: TestMap,
        index: usize,
    }

    let mut map = TestMap {
        data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
    };
    
    let entry = Entry { map, index: 1 };
    let (key, value) = entry.shift_remove_entry();
    assert_eq!(key, 2);
    assert_eq!(value, "two");
}

#[test]
#[should_panic]
fn test_shift_remove_entry_index_out_of_bounds() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.data.len() {
                let item = self.data.remove(index);
                Some(item)
            } else {
                None
            }
        }
    }

    struct Entry<K, V> {
        map: TestMap,
        index: usize,
    }

    let mut map = TestMap {
        data: vec![(1, "one".to_string()), (2, "two".to_string())],
    };
    
    let entry = Entry { map, index: 2 }; // This index is out of bounds
    entry.shift_remove_entry(); // This should panic
}

#[test]
fn test_shift_remove_entry_at_start() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.data.len() {
                let item = self.data.remove(index);
                Some(item)
            } else {
                None
            }
        }
    }

    struct Entry<K, V> {
        map: TestMap,
        index: usize,
    }

    let mut map = TestMap {
        data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
    };
    
    let entry = Entry { map, index: 0 };
    let (key, value) = entry.shift_remove_entry();
    assert_eq!(key, 1);
    assert_eq!(value, "one");
}

#[test]
fn test_shift_remove_entry_at_end() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, String)> {
            if index < self.data.len() {
                let item = self.data.remove(index);
                Some(item)
            } else {
                None
            }
        }
    }

    struct Entry<K, V> {
        map: TestMap,
        index: usize,
    }

    let mut map = TestMap {
        data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
    };
    
    let entry = Entry { map, index: 2 };
    let (key, value) = entry.shift_remove_entry();
    assert_eq!(key, 3);
    assert_eq!(value, "three");
}

