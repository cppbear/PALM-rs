// Answer 0

#[test]
fn test_state_heap_size_zero_byte_classes() {
    let transitions = Transitions {
        table: Vec::new(),
        num_byte_classes: 0,
    };
    assert_eq!(transitions.state_heap_size(), 0);
}

#[test]
fn test_state_heap_size_one_byte_class() {
    let transitions = Transitions {
        table: Vec::new(),
        num_byte_classes: 1,
    };
    assert_eq!(transitions.state_heap_size(), std::mem::size_of::<StatePtr>());
}

#[test]
fn test_state_heap_size_multiple_byte_classes() {
    let transitions = Transitions {
        table: Vec::new(),
        num_byte_classes: 5,
    };
    assert_eq!(transitions.state_heap_size(), 5 * std::mem::size_of::<StatePtr>());
}

