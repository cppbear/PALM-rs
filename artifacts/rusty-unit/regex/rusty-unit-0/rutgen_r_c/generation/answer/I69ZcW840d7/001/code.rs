// Answer 0

#[test]
fn test_next_valid_state() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            let size = STATE_MAX as usize + 1 + num_byte_classes;
            Transitions {
                table: vec![0; size],
                num_byte_classes,
            }
        }

        fn next(&self, si: StatePtr, cls: usize) -> StatePtr {
            self.table[si as usize + cls]
        }
    }

    let num_byte_classes = 4;
    let mut transitions = Transitions::new(num_byte_classes);
    
    transitions.table[0] = 10; // Setting up a test value
    let result = transitions.next(0, 0);
    assert_eq!(result, 10);
}

#[test]
#[should_panic]
fn test_next_invalid_state() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            let size = STATE_MAX as usize + 1 + num_byte_classes;
            Transitions {
                table: vec![0; size],
                num_byte_classes,
            }
        }

        fn next(&self, si: StatePtr, cls: usize) -> StatePtr {
            self.table[si as usize + cls]
        }
    }

    let num_byte_classes = 4;
    let transitions = Transitions::new(num_byte_classes);

    // This will panic since there's no entry at index `si + cls` (where si = 0, cls = 10).
    let _ = transitions.next(0, 10);
} 

#[test]
fn test_next_boundary() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            let size = STATE_MAX as usize + 1 + num_byte_classes;
            Transitions {
                table: vec![0; size],
                num_byte_classes,
            }
        }

        fn next(&self, si: StatePtr, cls: usize) -> StatePtr {
            self.table[si as usize + cls]
        }
    }

    let num_byte_classes = 4;
    let mut transitions = Transitions::new(num_byte_classes);
    
    transitions.table[STATE_MAX as usize] = 20; // Setting up a boundary value
    let result = transitions.next(STATE_MAX, 0);
    assert_eq!(result, 20);
}

