// Answer 0

#[test]
fn test_from_hashbrown_capacity_overflow() {
    use hashbrown::TryReserveError;

    let error = TryReserveError::CapacityOverflow;
    let result = TryReserveError::from_hashbrown(error);
    
    assert_eq!(result.kind, TryReserveErrorKind::CapacityOverflow);
}

#[test]
fn test_from_hashbrown_alloc_error() {
    use hashbrown::TryReserveError;
    use std::alloc::Layout;

    let layout = Layout::from_size_align(1, 1).unwrap();
    let error = TryReserveError::AllocError { layout: layout.clone() };
    
    let result = TryReserveError::from_hashbrown(error);
    
    assert_eq!(result.kind, TryReserveErrorKind::AllocError { layout });
}

