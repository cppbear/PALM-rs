// Answer 0

#[test]
fn test_from_hashbrown_alloc_error_zero_size() {
    let layout = alloc::alloc::Layout::from_size_align(0, 1).unwrap();
    let error = hashbrown::TryReserveError::AllocError { layout };
    let result = TryReserveError::from_hashbrown(error);
}

#[test]
fn test_from_hashbrown_alloc_error_one_size() {
    let layout = alloc::alloc::Layout::from_size_align(1, 1).unwrap();
    let error = hashbrown::TryReserveError::AllocError { layout };
    let result = TryReserveError::from_hashbrown(error);
}

#[test]
fn test_from_hashbrown_alloc_error_max_size() {
    let layout = alloc::alloc::Layout::from_size_align(usize::MAX, 1).unwrap();
    let error = hashbrown::TryReserveError::AllocError { layout };
    let result = TryReserveError::from_hashbrown(error);
}

