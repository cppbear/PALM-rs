// Answer 0

#[test]
fn test_shift_remove_index_success() {
    struct TestEntries {
        entries: Vec<(i32, i32)>, // Use i32 for both K and V types
    }

    impl TestEntries {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            Self { entries }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (i32, i32) {
            self.entries.remove(index)
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, i32)> {
            match self.entries.get(index) {
                Some(_) => {
                    self.shift_remove_finish(index);
                    Some(self.entries[index].clone())
                }
                None => None,
            }
        }
    }

    let mut test_entries = TestEntries::new(vec![(1, 2), (3, 4), (5, 6)]);

    let result = test_entries.shift_remove_index(1);
    assert_eq!(result, Some((3, 4))); // Expecting the entry at index 1 to be removed and returned
}

#[test]
fn test_shift_remove_index_boundary_low() {
    struct TestEntries {
        entries: Vec<(i32, i32)>,
    }

    impl TestEntries {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            Self { entries }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (i32, i32) {
            self.entries.remove(index)
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, i32)> {
            match self.entries.get(index) {
                Some(_) => {
                    self.shift_remove_finish(index);
                    Some(self.entries[index].clone())
                }
                None => None,
            }
        }
    }

    let mut test_entries = TestEntries::new(vec![(1, 2), (3, 4), (5, 6)]);

    let result = test_entries.shift_remove_index(0);
    assert_eq!(result, Some((1, 2))); // Expecting the entry at index 0 to be removed and returned
}

#[test]
fn test_shift_remove_index_boundary_high() {
    struct TestEntries {
        entries: Vec<(i32, i32)>,
    }

    impl TestEntries {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            Self { entries }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (i32, i32) {
            self.entries.remove(index)
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, i32)> {
            match self.entries.get(index) {
                Some(_) => {
                    self.shift_remove_finish(index);
                    Some(self.entries[index].clone())
                }
                None => None,
            }
        }
    }

    let mut test_entries = TestEntries::new(vec![(1, 2), (3, 4), (5, 6)]);

    let result = test_entries.shift_remove_index(2);
    assert_eq!(result, Some((5, 6))); // Expecting the entry at index 2 to be removed and returned
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    struct TestEntries {
        entries: Vec<(i32, i32)>,
    }

    impl TestEntries {
        fn new(entries: Vec<(i32, i32)>) -> Self {
            Self { entries }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (i32, i32) {
            self.entries.remove(index)
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, i32)> {
            match self.entries.get(index) {
                Some(_) => {
                    self.shift_remove_finish(index);
                    Some(self.entries[index].clone())
                }
                None => None,
            }
        }
    }

    let mut test_entries = TestEntries::new(vec![(1, 2), (3, 4), (5, 6)]);

    let result = test_entries.shift_remove_index(3);
    assert_eq!(result, None); // Expecting None as index 3 is out of bounds
}

