// Answer 0

#[test]
fn test_clear_transitions_table() {
    struct Transitions {
        table: Vec<u32>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            Transitions {
                table: vec![1, 2, 3, 4, 5],
                num_byte_classes,
            }
        }

        fn clear(&mut self) {
            self.table.clear();
        }
    }

    let mut transitions = Transitions::new(256);
    assert_eq!(transitions.table.len(), 5);
    transitions.clear();
    assert_eq!(transitions.table.len(), 0);
}

#[test]
fn test_clear_empty_transitions_table() {
    struct Transitions {
        table: Vec<u32>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> Transitions {
            Transitions {
                table: Vec::new(),
                num_byte_classes,
            }
        }

        fn clear(&mut self) {
            self.table.clear();
        }
    }

    let mut transitions = Transitions::new(256);
    assert_eq!(transitions.table.len(), 0);
    transitions.clear();
    assert_eq!(transitions.table.len(), 0);
}

