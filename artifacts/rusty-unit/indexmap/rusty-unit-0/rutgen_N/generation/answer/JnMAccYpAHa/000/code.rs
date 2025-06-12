// Answer 0

#[test]
fn test_index() {
    struct TestEntry {
        index: std::cell::RefCell<usize>,
    }

    impl TestEntry {
        fn new(index: usize) -> Self {
            TestEntry {
                index: std::cell::RefCell::new(index),
            }
        }
    
        fn index(&self) -> usize {
            *self.index.borrow()
        }
    }

    let entry = TestEntry::new(5);
    assert_eq!(entry.index(), 5);
}

#[test]
fn test_index_zero() {
    struct TestEntry {
        index: std::cell::RefCell<usize>,
    }

    impl TestEntry {
        fn new(index: usize) -> Self {
            TestEntry {
                index: std::cell::RefCell::new(index),
            }
        }

        fn index(&self) -> usize {
            *self.index.borrow()
        }
    }

    let entry = TestEntry::new(0);
    assert_eq!(entry.index(), 0);
}

