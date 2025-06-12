// Answer 0

#[test]
fn test_num_states_empty_table() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: Vec::new(),
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    let transitions = Transitions::new(1);
    assert_eq!(transitions.num_states(), 0);
}

#[test]
fn test_num_states_one_state() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: vec![0], // one state
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    let transitions = Transitions::new(1);
    assert_eq!(transitions.num_states(), 1);
}

#[test]
fn test_num_states_multiple_states() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: vec![0; 6], // six states total with 1 byte class
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    let transitions = Transitions::new(1);
    assert_eq!(transitions.num_states(), 6);
}

#[test]
fn test_num_states_two_byte_classes() {
    struct Transitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: vec![0; 8], // four states with 2 byte classes
                num_byte_classes,
            }
        }

        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    let transitions = Transitions::new(2);
    assert_eq!(transitions.num_states(), 4);
}

