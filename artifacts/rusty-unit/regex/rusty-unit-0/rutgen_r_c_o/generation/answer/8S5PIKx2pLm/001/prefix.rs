// Answer 0

#[test]
fn test_add_exceeds_state_max() {
    let mut transitions = Transitions::new(1);
    for _ in 0..=STATE_MAX as usize {
        transitions.add();
    }
    let result = transitions.add();
}

#[test]
fn test_add_exceeds_state_max_with_multiple_byte_classes() {
    let mut transitions = Transitions::new(10);
    for _ in 0..=STATE_MAX as usize {
        transitions.add();
    }
    let result = transitions.add();
}

#[test]
fn test_add_exceeds_state_max_with_edge_case() {
    let mut transitions = Transitions::new(2);
    for _ in 0..(STATE_MAX as usize + 1) {
        transitions.add();
    }
    let result = transitions.add();
}

