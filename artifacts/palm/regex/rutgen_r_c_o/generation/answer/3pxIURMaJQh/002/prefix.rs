// Answer 0

#[test]
fn test_next_unchecked_valid_si_cls() {
    let num_byte_classes = 5;
    let mut transitions = Transitions::new(num_byte_classes);
    let state_index: StatePtr = transitions.add().unwrap();
    let class_index = num_byte_classes - 1; // valid class index
    unsafe {
        let _ = transitions.next_unchecked(state_index, class_index);
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_invalid_cls() {
    let num_byte_classes = 5;
    let mut transitions = Transitions::new(num_byte_classes);
    let state_index: StatePtr = transitions.add().unwrap();
    let class_index = num_byte_classes; // invalid class index
    unsafe {
        let _ = transitions.next_unchecked(state_index, class_index);
    }
}

#[test]
fn test_next_unchecked_boundary_si() {
    let num_byte_classes = 3;
    let mut transitions = Transitions::new(num_byte_classes);
    let state_index: StatePtr = transitions.add().unwrap(); 
    let class_index = 2; // valid class index
    unsafe {
        let _ = transitions.next_unchecked(state_index, class_index);
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_out_of_bounds_si() {
    let num_byte_classes = 2;
    let mut transitions = Transitions::new(num_byte_classes);
    let state_index: StatePtr = STATE_MAX + 1; // out of bounds state index
    let class_index = 1; // valid class index
    unsafe {
        let _ = transitions.next_unchecked(state_index, class_index);
    }
}

