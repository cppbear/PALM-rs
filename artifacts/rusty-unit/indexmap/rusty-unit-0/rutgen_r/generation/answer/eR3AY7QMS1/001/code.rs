// Answer 0

#[test]
fn test_shift_remove_empty_map() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn shift_remove(&mut self, index: usize) -> Option<i32> {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries[index].1;
            self.entries.remove(index);
            Some(value)
        }
    }

    let mut map = TestMap { entries: vec![] };
    let result = std::panic::catch_unwind(|| map.shift_remove(0));
    assert!(result.is_err());
}

#[test]
fn test_shift_remove_single_entry() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn shift_remove(&mut self, index: usize) -> Option<i32> {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries[index].1;
            self.entries.remove(index);
            Some(value)
        }
    }

    let mut map = TestMap { entries: vec![(1, 10)] };
    let result = map.shift_remove(0).unwrap();
    assert_eq!(result, 10);
    assert!(map.entries.is_empty());
}

#[test]
fn test_shift_remove_multiple_entries() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn shift_remove(&mut self, index: usize) -> Option<i32> {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries[index].1;
            self.entries.remove(index);
            Some(value)
        }
    }

    let mut map = TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] };
    let result = map.shift_remove(1).unwrap();
    assert_eq!(result, 20);
    assert_eq!(map.entries, vec![(1, 10), (3, 30)]);
}

#[test]
fn test_shift_remove_last_entry() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn shift_remove(&mut self, index: usize) -> Option<i32> {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries[index].1;
            self.entries.remove(index);
            Some(value)
        }
    }

    let mut map = TestMap { entries: vec![(1, 10), (2, 20)] };
    let result = map.shift_remove(1).unwrap();
    assert_eq!(result, 20);
    assert_eq!(map.entries, vec![(1, 10)]);
}

