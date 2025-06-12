// Answer 0

#[test]
fn test_next_unchecked_valid_case() {
    let num_byte_classes = 3;
    let mut transitions = Transitions::new(num_byte_classes);
    
    let state1 = transitions.add().unwrap();
    let state2 = transitions.add().unwrap();
    transitions.set_next(state1, 0, state2);
    
    unsafe {
        transitions.next_unchecked(state1, 0);
    }
}

#[test]
fn test_next_unchecked_edge_case() {
    let num_byte_classes = 2;
    let mut transitions = Transitions::new(num_byte_classes);
    
    let state1 = transitions.add().unwrap();
    let state2 = transitions.add().unwrap();
    transitions.set_next(state1, 1, state2);
    
    unsafe {
        transitions.next_unchecked(state1, 1);
    }
}

#[test]
fn test_next_unchecked_boundary_case() {
    let num_byte_classes = 4;
    let mut transitions = Transitions::new(num_byte_classes);
    
    let state1 = transitions.add().unwrap();
    let state2 = transitions.add().unwrap();
    transitions.set_next(state1, 3, state2);
    
    unsafe {
        transitions.next_unchecked(state1, 3);
    }
}

#[test]
fn test_next_unchecked_multiple_states() {
    let num_byte_classes = 5;
    let mut transitions = Transitions::new(num_byte_classes);
    
    let state1 = transitions.add().unwrap();
    let state2 = transitions.add().unwrap();
    let state3 = transitions.add().unwrap();
    transitions.set_next(state1, 0, state2);
    transitions.set_next(state1, 1, state3);
    
    unsafe {
        transitions.next_unchecked(state1, 0);
        transitions.next_unchecked(state1, 1);
    }
}

