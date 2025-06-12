// Answer 0

#[test]
fn test_shift_remove_empty() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn shift_remove_entry(&mut self, index: usize) -> (usize, usize) {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries.remove(index);
            (index, value.1)
        }
        
        fn shift_remove(&mut self, index: usize) -> usize {
            self.shift_remove_entry(index).1
        }
    }
    
    let mut map = TestMap { entries: Vec::new() };

    // Attempting to remove from an empty map should panic
    let result = std::panic::catch_unwind(|| {
        map.shift_remove(0);
    });
    assert!(result.is_err());
}

#[test]
fn test_shift_remove_single_element() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn shift_remove_entry(&mut self, index: usize) -> (usize, usize) {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries.remove(index);
            (index, value.1)
        }

        fn shift_remove(&mut self, index: usize) -> usize {
            self.shift_remove_entry(index).1
        }
    }

    let mut map = TestMap { entries: vec![(1, 10)] };

    // Remove the single element
    let value = map.shift_remove(0);
    assert_eq!(value, 10);
    assert!(map.entries.is_empty());
}

#[test]
fn test_shift_remove_multiple_elements() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn shift_remove_entry(&mut self, index: usize) -> (usize, usize) {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries.remove(index);
            (index, value.1)
        }

        fn shift_remove(&mut self, index: usize) -> usize {
            self.shift_remove_entry(index).1
        }
    }

    let mut map = TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] };

    // Remove an element and check the results
    let value = map.shift_remove(1);
    assert_eq!(value, 20);
    assert_eq!(map.entries, vec![(1, 10), (3, 30)]);
}

#[test]
fn test_shift_remove_out_of_bounds() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }
    
    impl TestMap {
        fn shift_remove_entry(&mut self, index: usize) -> (usize, usize) {
            if index >= self.entries.len() {
                panic!("Index out of bounds");
            }
            let value = self.entries.remove(index);
            (index, value.1)
        }

        fn shift_remove(&mut self, index: usize) -> usize {
            self.shift_remove_entry(index).1
        }
    }

    let mut map = TestMap { entries: vec![(1, 10), (2, 20)] };

    // Attempting to remove an out of bounds index should panic
    let result = std::panic::catch_unwind(|| {
        map.shift_remove(3);
    });
    assert!(result.is_err());
}

