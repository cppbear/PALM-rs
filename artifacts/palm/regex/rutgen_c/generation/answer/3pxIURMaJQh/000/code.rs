// Answer 0

#[test]
fn test_next_unchecked_valid_access() {
    struct TestTransitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TestTransitions {
        fn new(num_byte_classes: usize) -> Self {
            let table_size = 10; // Example size for the table
            let table = (0..table_size as StatePtr).collect();
            TestTransitions { table, num_byte_classes }
        }

        fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            unsafe {
                debug_assert!((si as usize) < self.table.len());
                debug_assert!(cls < self.num_byte_classes);
                *self.table.get_unchecked(si as usize + cls)
            }
        }
    }

    let transitions = TestTransitions::new(5);
    let result = transitions.next_unchecked(0, 0);
    assert_eq!(result, 0); // 0 is at index 0

    let result = transitions.next_unchecked(2, 2);
    assert_eq!(result, 4); // 4 is at index 4 (2 + 2)
}

#[test]
#[should_panic]
fn test_next_unchecked_invalid_state_index() {
    struct TestTransitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TestTransitions {
        fn new(num_byte_classes: usize) -> Self {
            let table_size = 10; // Example size for the table
            let table = (0..table_size as StatePtr).collect();
            TestTransitions { table, num_byte_classes }
        }

        fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            unsafe {
                debug_assert!((si as usize) < self.table.len());
                debug_assert!(cls < self.num_byte_classes);
                *self.table.get_unchecked(si as usize + cls)
            }
        }
    }

    let transitions = TestTransitions::new(5);
    let _ = transitions.next_unchecked(10, 0); // Index out of bounds
}

#[test]
#[should_panic]
fn test_next_unchecked_invalid_class_index() {
    struct TestTransitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TestTransitions {
        fn new(num_byte_classes: usize) -> Self {
            let table_size = 10; // Example size for the table
            let table = (0..table_size as StatePtr).collect();
            TestTransitions { table, num_byte_classes }
        }

        fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            unsafe {
                debug_assert!((si as usize) < self.table.len());
                debug_assert!(cls < self.num_byte_classes);
                *self.table.get_unchecked(si as usize + cls)
            }
        }
    }

    let transitions = TestTransitions::new(5);
    let _ = transitions.next_unchecked(0, 5); // Class index out of bounds
}

