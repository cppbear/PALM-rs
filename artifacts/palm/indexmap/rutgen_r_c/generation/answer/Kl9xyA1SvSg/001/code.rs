// Answer 0

#[test]
fn test_move_index_valid_upward_shift() {
    // Helper struct to mimic the functionality without external dependencies
    struct TestIndexSet {
        values: Vec<i32>,
    }
    
    impl TestIndexSet {
        fn new() -> Self {
            TestIndexSet { values: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let value = self.values.remove(from);
            self.values.insert(to, value);
        }

        fn as_slice(&self) -> &[i32] {
            &self.values
        }
    }

    let mut set = TestIndexSet::new();
    set.move_index(2, 0);
    assert_eq!(set.as_slice(), &[3, 1, 2, 4, 5]);
}

#[test]
fn test_move_index_valid_downward_shift() {
    struct TestIndexSet {
        values: Vec<i32>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            TestIndexSet { values: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let value = self.values.remove(from);
            self.values.insert(to, value);
        }

        fn as_slice(&self) -> &[i32] {
            &self.values
        }
    }

    let mut set = TestIndexSet::new();
    set.move_index(0, 3);
    assert_eq!(set.as_slice(), &[2, 3, 4, 1, 5]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_from_out_of_bounds() {
    struct TestIndexSet {
        values: Vec<i32>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            TestIndexSet { values: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let value = self.values.remove(from);
            self.values.insert(to, value);
        }
    }

    let mut set = TestIndexSet::new();
    set.move_index(5, 0); // Out of bounds access
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_to_out_of_bounds() {
    struct TestIndexSet {
        values: Vec<i32>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            TestIndexSet { values: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let value = self.values.remove(from);
            self.values.insert(to, value);
        }
    }

    let mut set = TestIndexSet::new();
    set.move_index(1, 10); // Out of bounds access
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_both_out_of_bounds() {
    struct TestIndexSet {
        values: Vec<i32>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            TestIndexSet { values: vec![1, 2, 3, 4, 5] }
        }

        fn move_index(&mut self, from: usize, to: usize) {
            let value = self.values.remove(from);
            self.values.insert(to, value);
        }
    }

    let mut set = TestIndexSet::new();
    set.move_index(10, 10); // Both indices out of bounds
}

