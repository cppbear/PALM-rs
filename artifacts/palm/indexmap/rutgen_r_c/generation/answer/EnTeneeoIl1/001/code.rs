// Answer 0

#[test]
fn test_from_hashbrown_alloc_error() {
    use alloc::alloc::Layout;
    use hashbrown::TryReserveError;

    let layout = Layout::from_size_align(1, 1).unwrap();
    let error = TryReserveError::AllocError { layout };

    let result = TryReserveError::from_hashbrown(error);
    
    assert!(matches!(result.kind, TryReserveErrorKind::AllocError { .. }));
}

#[test]
fn test_from_hashbrown_capacity_overflow() {
    use hashbrown::TryReserveError;

    let error = TryReserveError::CapacityOverflow;

    let result = TryReserveError::from_hashbrown(error);
    
    assert!(matches!(result.kind, TryReserveErrorKind::CapacityOverflow));
}

