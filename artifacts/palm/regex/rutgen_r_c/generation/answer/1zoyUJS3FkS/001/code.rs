// Answer 0

#[test]
fn test_new_transitions_zero_byte_classes() {
    let transitions = Transitions::new(0);
    assert_eq!(transitions.table.len(), 0);
    assert_eq!(transitions.num_byte_classes, 0);
}

#[test]
fn test_new_transitions_one_byte_class() {
    let transitions = Transitions::new(1);
    assert_eq!(transitions.table.len(), 0);
    assert_eq!(transitions.num_byte_classes, 1);
}

#[test]
fn test_new_transitions_many_byte_classes() {
    let transitions = Transitions::new(10);
    assert_eq!(transitions.table.len(), 0);
    assert_eq!(transitions.num_byte_classes, 10);
}

#[test]
fn test_new_transitions_large_byte_classes() {
    let transitions = Transitions::new(1000);
    assert_eq!(transitions.table.len(), 0);
    assert_eq!(transitions.num_byte_classes, 1000);
}

