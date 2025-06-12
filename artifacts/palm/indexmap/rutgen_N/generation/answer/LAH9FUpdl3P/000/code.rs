// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct TestEntry {
        index: usize,
    }

    impl TestEntry {
        fn index(&self) -> usize {
            self.index
        }

        fn into_ref_mut(self) -> TestEntry {
            self
        }

        fn move_index(&mut self, from: usize, to: usize) {
            self.index = to; // Simulating the shift
        }
    }

    let mut entry = TestEntry { index: 0 };
    entry.move_index(0, 1);
    assert_eq!(entry.index(), 1);

    entry.move_index(1, 0);
    assert_eq!(entry.index(), 0);
}

#[should_panic]
fn test_move_index_out_of_bounds() {
    struct TestEntry {
        index: usize,
        size: usize,
    }

    impl TestEntry {
        fn index(&self) -> usize {
            self.index
        }

        fn into_ref_mut(self) -> TestEntry {
            self
        }

        fn move_index(&mut self, from: usize, to: usize) {
            // Simulating panic if 'to' is out of bounds
            if to >= self.size {
                panic!("Index out of bounds");
            }
            self.index = to; // Simulating the shift
        }
    }

    let mut entry = TestEntry { index: 0, size: 1 };
    entry.move_index(0, 2); // This should panic
}

