// Answer 0

#[test]
fn test_add_exceeds_state_max() {
    struct TestTransitions {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl Transitions {
        fn new(num_byte_classes: usize) -> TestTransitions {
            TestTransitions { table: Vec::new(), num_byte_classes }
        }

        fn add(&mut self) -> Option<StatePtr> {
            let si = self.table.len();
            if si > STATE_MAX as usize {
                return None;
            }
            self.table.extend(repeat(STATE_UNKNOWN).take(self.num_byte_classes));
            Some(usize_to_u32(si))
        }
    }

    const NUM_BYTE_CLASSES: usize = 2;

    let mut transitions = TestTransitions::new(NUM_BYTE_CLASSES);
    // Fill the table to the maximum size before it exceeds STATE_MAX
    for _ in 0..=STATE_MAX as usize {
        transitions.table.push(STATE_UNKNOWN);
    }

    // Now the size is STATE_MAX + 1, the next add should return None
    let result = transitions.add();
    assert_eq!(result, None);
}

