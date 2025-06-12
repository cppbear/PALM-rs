// Answer 0

#[test]
fn test_flags_with_min_value() {
    let state = State {
        data: Box::new([0]),
    };
    state.flags();
}

#[test]
fn test_flags_with_mid_value() {
    let state = State {
        data: Box::new([127]),
    };
    state.flags();
}

#[test]
fn test_flags_with_max_value() {
    let state = State {
        data: Box::new([255]),
    };
    state.flags();
}

#[test]
fn test_flags_with_consecutive_values() {
    let state = State {
        data: Box::new([1]),
    };
    state.flags();
    
    let state = State {
        data: Box::new([2]),
    };
    state.flags();
    
    let state = State {
        data: Box::new([3]),
    };
    state.flags();
}

#[test]
fn test_flags_with_random_values() {
    let values = [45, 99, 150, 200, 250];
    for &value in &values {
        let state = State {
            data: Box::new([value]),
        };
        state.flags();
    }
}

#[test]
#[should_panic]
fn test_flags_with_empty_data() {
    let state = State {
        data: Box::new([]),
    };
    state.flags();
}

