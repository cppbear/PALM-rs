// Answer 0

#[test]
fn test_next_unchecked_valid_index() {
    let num_byte_classes = 5;
    let mut transitions = Transitions::new(num_byte_classes);
    // Populate the table with at least num_byte_classes + 1 entries
    for _ in 0..num_byte_classes + 1 {
        transitions.add();
    }

    unsafe {
        let result = transitions.next_unchecked(0, 0);
        assert_eq!(result, transitions.table[0]); // Expect the first entry
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_invalid_state_index() {
    let num_byte_classes = 5;
    let mut transitions = Transitions::new(num_byte_classes);
    for _ in 0..num_byte_classes + 1 {
        transitions.add();
    }

    unsafe {
        // si is equal to the length of the table, which should panic
        let _ = transitions.next_unchecked(num_byte_classes as StatePtr, 0);
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_invalid_class_index() {
    let num_byte_classes = 5;
    let mut transitions = Transitions::new(num_byte_classes);
    for _ in 0..num_byte_classes + 1 {
        transitions.add();
    }

    unsafe {
        // cls is equal to num_byte_classes, which should panic
        let _ = transitions.next_unchecked(0, num_byte_classes);
    }
}

