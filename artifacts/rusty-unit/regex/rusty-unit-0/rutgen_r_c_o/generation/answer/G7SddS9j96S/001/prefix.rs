// Answer 0

#[test]
fn test_state_heap_size_min_byte_classes() {
    let num_byte_classes = 1;
    let transitions = Transitions::new(num_byte_classes);
    let size = transitions.state_heap_size();
}

#[test]
fn test_state_heap_size_mid_byte_classes() {
    let num_byte_classes = 10;
    let transitions = Transitions::new(num_byte_classes);
    let size = transitions.state_heap_size();
}

#[test]
fn test_state_heap_size_max_byte_classes() {
    let num_byte_classes = (STATE_MAX / mem::size_of::<StatePtr>);
    let transitions = Transitions::new(num_byte_classes);
    let size = transitions.state_heap_size();
}

#[test]
#[should_panic]
fn test_state_heap_size_exceeding_max_byte_classes() {
    let num_byte_classes = (STATE_MAX / mem::size_of::<StatePtr>) + 1;
    let transitions = Transitions::new(num_byte_classes);
    let size = transitions.state_heap_size();
}

