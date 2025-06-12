// Answer 0

#[test]
fn test_set_next_valid_transition() {
    let mut transitions = Transitions::new(5); // Assuming 5 byte classes

    // Add a state and verify its index
    let state_index = transitions.add().expect("Failed to add a state");
    
    // Set a valid transition
    transitions.set_next(state_index, 2, 10); // Set transition from state_index with class 2 to state 10

    // Verify the transition was set correctly
    assert_eq!(transitions.next(state_index, 2), 10);
}

#[test]
#[should_panic(expected = "index out of bounds")] // Expected panic
fn test_set_next_out_of_bounds_state_index() {
    let mut transitions = Transitions::new(5); // Assuming 5 byte classes 

    // Attempt to access an invalid state index (e.g., 1000)
    let state_index = 1000;
    
    // This should panic as we are out of the bounds of the table
    transitions.set_next(state_index, 2, 10);
}

#[test]
#[should_panic(expected = "index out of bounds")] // Expected panic
fn test_set_next_invalid_class_index() {
    let mut transitions = Transitions::new(5); // Assuming 5 byte classes
    let state_index = transitions.add().expect("Failed to add a state");

    // Attempt to access an invalid class index (e.g., 10)
    let class_index = 10;

    // This should panic as class_index is out of the defined range
    transitions.set_next(state_index, class_index, 10);
}

