// Answer 0

#[test]
fn test_new_transitions_zero_byte_classes() {
    let transitions = new(0);
    assert_eq!(transitions.table, vec![]);
    assert_eq!(transitions.num_byte_classes, 0);
}

#[test]
fn test_new_transitions_one_byte_class() {
    let transitions = new(1);
    assert_eq!(transitions.table, vec![]);
    assert_eq!(transitions.num_byte_classes, 1);
}

#[test]
fn test_new_transitions_multiple_byte_classes() {
    let num_byte_classes = 10;
    let transitions = new(num_byte_classes);
    assert_eq!(transitions.table, vec![]);
    assert_eq!(transitions.num_byte_classes, num_byte_classes);
}

#[test]
fn test_new_transitions_large_byte_classes() {
    let num_byte_classes = 100_000;
    let transitions = new(num_byte_classes);
    assert_eq!(transitions.table, vec![]);
    assert_eq!(transitions.num_byte_classes, num_byte_classes);
}

