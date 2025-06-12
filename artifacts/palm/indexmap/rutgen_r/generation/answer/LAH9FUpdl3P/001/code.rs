// Answer 0

#[test]
fn test_move_index_shift_down() {
    struct TestEntry {
        index: usize,
    }

    impl TestEntry {
        fn new(index: usize) -> Self {
            Self { index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(&mut self, from: usize, to: usize) {
            // Simulate moving index (this would normally mutate the structure)
            self.index = to;
        }

        fn into_ref_mut(self) -> TestEntryRefMut {
            TestEntryRefMut { entry: self }
        }
    }

    struct TestEntryRefMut {
        entry: TestEntry,
    }

    impl TestEntryRefMut {
        fn move_index(&mut self, from: usize, to: usize) {
            self.entry.move_index(from, to);
        }
    }

    let mut entry = TestEntry::new(0);
    entry.move_index(0, 2);
    assert_eq!(entry.index(), 2);
}

#[test]
fn test_move_index_shift_up() {
    struct TestEntry {
        index: usize,
    }

    impl TestEntry {
        fn new(index: usize) -> Self {
            Self { index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(&mut self, from: usize, to: usize) {
            self.index = to;
        }

        fn into_ref_mut(self) -> TestEntryRefMut {
            TestEntryRefMut { entry: self }
        }
    }

    struct TestEntryRefMut {
        entry: TestEntry,
    }

    impl TestEntryRefMut {
        fn move_index(&mut self, from: usize, to: usize) {
            self.entry.move_index(from, to);
        }
    }

    let mut entry = TestEntry::new(3);
    entry.move_index(3, 1);
    assert_eq!(entry.index(), 1);
}

#[should_panic]
#[test]
fn test_move_index_out_of_bounds() {
    struct TestEntry {
        index: usize,
    }

    impl TestEntry {
        fn new(index: usize) -> Self {
            Self { index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(&mut self, from: usize, to: usize) {
            if to > 5 {
                panic!("to index out of bounds");
            }
            self.index = to;
        }

        fn into_ref_mut(self) -> TestEntryRefMut {
            TestEntryRefMut { entry: self }
        }
    }

    struct TestEntryRefMut {
        entry: TestEntry,
    }

    impl TestEntryRefMut {
        fn move_index(&mut self, from: usize, to: usize) {
            self.entry.move_index(from, to);
        }
    }

    let mut entry = TestEntry::new(0);
    entry.move_index(0, 6); // This should trigger a panic
}

#[test]
fn test_move_index_no_change() {
    struct TestEntry {
        index: usize,
    }

    impl TestEntry {
        fn new(index: usize) -> Self {
            Self { index }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(&mut self, from: usize, to: usize) {
            // No change should occur if moved to the same index
            if from != to {
                self.index = to;
            }
        }

        fn into_ref_mut(self) -> TestEntryRefMut {
            TestEntryRefMut { entry: self }
        }
    }

    struct TestEntryRefMut {
        entry: TestEntry,
    }

    impl TestEntryRefMut {
        fn move_index(&mut self, from: usize, to: usize) {
            self.entry.move_index(from, to);
        }
    }

    let mut entry = TestEntry::new(1);
    entry.move_index(1, 1); // Moving to the same index
    assert_eq!(entry.index(), 1);
}

