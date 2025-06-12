// Answer 0

#[test]
fn test_next_unchecked_out_of_bounds_sanity() {
    let mut transitions = Transitions::new(5); // Assuming we have 5 byte classes
    transitions.table = vec![0, 1, 2, 3]; // Table initialized to have 4 states

    let si = transitions.table.len() as StatePtr; // si set to the length of the table to trigger panic
    let cls = 0;

    // Since `next_unchecked` is an unsafe method, we need to wrap it in an unsafe block
    // to test the panic condition
    let result = std::panic::catch_unwind(|| {
        unsafe {
            transitions.next_unchecked(si, cls);
        }
    });

    assert!(result.is_err()); // We expect this to panic
}

#[test]
fn test_next_unchecked_byte_class_out_of_bounds() {
    let mut transitions = Transitions::new(5); // 5 byte classes
    transitions.table = vec![0, 1, 2, 3]; // Table initialized with 4 states

    let si = 0; // valid state index
    let cls = transitions.num_byte_classes; // cls set to the number of byte classes to trigger panic

    // Testing the panic condition
    let result = std::panic::catch_unwind(|| {
        unsafe {
            transitions.next_unchecked(si, cls);
        }
    });

    assert!(result.is_err()); // We expect this to panic
}

