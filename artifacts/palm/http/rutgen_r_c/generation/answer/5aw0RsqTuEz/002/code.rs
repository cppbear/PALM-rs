// Answer 0

#[test]
fn test_try_reserve_success() {
    let mut map = HeaderMap::with_capacity(5);
    let result = map.try_reserve(10);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_edge_case() {
    let mut map = HeaderMap::with_capacity(12);
    let result = map.try_reserve(u16::MAX as usize - 11); // Should succeed, not overflow
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_try_reserve_exceeds_max_size() {
    let mut map = HeaderMap::with_capacity(MAX_SIZE);
    let _ = map.try_reserve(1).unwrap(); // This should panic since it can't exceed MAX_SIZE
}

#[test]
#[should_panic]
fn test_try_reserve_overflow() {
    let mut map = HeaderMap::with_capacity(0);
    let _ = map.try_reserve((u16::MAX as usize)); // This should overflow and panic
}

#[test]
fn test_try_reserve_no_growth_needed() {
    let mut map = HeaderMap::with_capacity(5);
    map.try_reserve(0).unwrap(); // No additional headers, should not fail
    assert!(map.len() == 0);
}

