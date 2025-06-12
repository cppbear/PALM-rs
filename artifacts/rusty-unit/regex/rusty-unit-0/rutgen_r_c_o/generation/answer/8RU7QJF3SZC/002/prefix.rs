// Answer 0

#[test]
fn test_fmt_empty_state() {
    let transitions = Transitions::new(5);
    let mut output = String::new();
    let _ = transitions.fmt(&mut output);
}

#[test]
fn test_fmt_single_state() {
    let mut transitions = Transitions::new(3);
    transitions.table.push(STATE_START);
    let mut output = String::new();
    let _ = transitions.fmt(&mut output);
}

#[test]
fn test_fmt_two_states() {
    let mut transitions = Transitions::new(4);
    transitions.table.push(STATE_START);
    transitions.table.push(STATE_DEAD);
    let mut output = String::new();
    let _ = transitions.fmt(&mut output);
}

#[test]
fn test_fmt_large_num_states() {
    let mut transitions = Transitions::new(10);
    (0..20).for_each(|_| transitions.table.push(STATE_QUIT));
    let mut output = String::new();
    let _ = transitions.fmt(&mut output);
}

#[test]
#[should_panic]
fn test_fmt_out_of_bounds_state_lower() {
    let transitions = Transitions::new(3);
    let mut output = String::new();
    // This should panic since num_states() is 0
    let _ = transitions.fmt(&mut output);
}

#[test]
#[should_panic]
fn test_fmt_out_of_bounds_state_upper() {
    let mut transitions = Transitions::new(2);
    transitions.table.push(STATE_START);
    let mut output = String::new();
    // This should panic since num_states() is 1 and si is incremented
    let _ = transitions.fmt(&mut output);
}

