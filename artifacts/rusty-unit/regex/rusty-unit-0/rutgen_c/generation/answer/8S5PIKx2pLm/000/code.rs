// Answer 0

#[test]
fn test_add_state_with_available_space() {
    struct TransitionsTest {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TransitionsTest {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: Vec::new(),
                num_byte_classes,
            }
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

    let mut transitions = TransitionsTest::new(2);
    let state_ptr = transitions.add();
    assert_eq!(state_ptr, Some(0));
    assert_eq!(transitions.table.len(), 2);
}

#[test]
fn test_add_state_reaching_max_capacity() {
    struct TransitionsTest {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TransitionsTest {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: Vec::new(),
                num_byte_classes,
            }
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

    let mut transitions = TransitionsTest::new(2);
    for _ in 0..(STATE_MAX as usize / 2) {
        transitions.add();
    }
    let state_ptr = transitions.add();
    assert_eq!(state_ptr, None);
}

#[test]
fn test_add_state_when_capacity_is_full() {
    struct TransitionsTest {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl TransitionsTest {
        fn new(num_byte_classes: usize) -> Self {
            Self {
                table: Vec::new(),
                num_byte_classes,
            }
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

    let mut transitions = TransitionsTest::new(2);
    for _ in 0..=STATE_MAX as usize {
        transitions.add();
    }
    let state_ptr = transitions.add();
    assert_eq!(state_ptr, None);
}

