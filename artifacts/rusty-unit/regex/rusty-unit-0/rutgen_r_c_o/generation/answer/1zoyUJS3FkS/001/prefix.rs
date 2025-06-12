// Answer 0

#[test]
fn test_new_transitions_zero_byte_classes() {
    let transitions = new(0);
}

#[test]
fn test_new_transitions_one_byte_class() {
    let transitions = new(1);
}

#[test]
fn test_new_transitions_two_byte_classes() {
    let transitions = new(2);
}

#[test]
fn test_new_transitions_max_byte_classes() {
    let transitions = new(32);
}

#[test]
fn test_new_transitions_multiple_byte_classes() {
    let transitions = new(16);
}

