// Answer 0

#[test]
fn test_set_next() {
    struct TestTransitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }
    
    impl TestTransitions {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: vec![STATE_UNKNOWN; 10], // Initialize with 10 slots
                num_byte_classes,
            }
        }

        fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {
            self.table[si as usize + cls] = next;
        }

        fn next(&self, si: StatePtr, cls: usize) -> StatePtr {
            self.table[si as usize + cls]
        }
    }

    let mut transitions = TestTransitions::new(2);

    transitions.set_next(0, 0, STATE_MATCH);
    assert_eq!(transitions.next(0, 0), STATE_MATCH);

    transitions.set_next(0, 1, STATE_DEAD);
    assert_eq!(transitions.next(0, 1), STATE_DEAD);
}

#[test]
fn test_set_next_boundary() {
    struct TestTransitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TestTransitions {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: vec![STATE_UNKNOWN; 10], // Initialize with 10 slots
                num_byte_classes,
            }
        }

        fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {
            self.table[si as usize + cls] = next;
        }

        fn next(&self, si: StatePtr, cls: usize) -> StatePtr {
            self.table[si as usize + cls]
        }
    }

    let mut transitions = TestTransitions::new(2);

    // Test setting a transition at the upper boundary
    transitions.set_next(8, 0, STATE_QUIT);
    assert_eq!(transitions.next(8, 0), STATE_QUIT);

    // Test setting beyond the boundary (this should normally panic or be a no-op)
    transitions.set_next(9, 0, STATE_DEAD);
    assert_eq!(transitions.next(9, 0), STATE_UNKNOWN); // verifiy it remains unknown unless specifically set
}

