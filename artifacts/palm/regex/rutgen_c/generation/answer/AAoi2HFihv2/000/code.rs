// Answer 0

#[test]
fn test_flags() {
    let state_data: Box<[u8]> = Box::new([5]);
    let state = State { data: state_data };

    let flags = state.flags();
    
    assert_eq!(flags.0, 5);
}

#[test]
fn test_flags_zero() {
    let state_data: Box<[u8]> = Box::new([0]);
    let state = State { data: state_data };

    let flags = state.flags();
    
    assert_eq!(flags.0, 0);
}

#[test]
fn test_flags_boundary() {
    let state_data: Box<[u8]> = Box::new([u8::MAX]);
    let state = State { data: state_data };

    let flags = state.flags();
    
    assert_eq!(flags.0, u8::MAX);
}

