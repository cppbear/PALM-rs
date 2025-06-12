// Answer 0

#[test]
fn test_new_transitions() {
    let num_byte_classes = 10;
    let transitions = Transitions::new(num_byte_classes);
    assert_eq!(transitions.num_byte_classes, num_byte_classes);
    assert_eq!(transitions.table.len(), 0);
}

#[test]
fn test_new_transitions_with_zero_classes() {
    let num_byte_classes = 0;
    let transitions = Transitions::new(num_byte_classes);
    assert_eq!(transitions.num_byte_classes, num_byte_classes);
    assert_eq!(transitions.table.len(), 0);
}

#[test]
fn test_new_transitions_with_large_classes() {
    let num_byte_classes = 1000;
    let transitions = Transitions::new(num_byte_classes);
    assert_eq!(transitions.num_byte_classes, num_byte_classes);
    assert_eq!(transitions.table.len(), 0);
}

