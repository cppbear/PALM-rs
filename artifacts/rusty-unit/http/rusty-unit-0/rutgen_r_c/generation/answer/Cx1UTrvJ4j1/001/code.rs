// Answer 0

#[test]
fn test_reserve_with_small_additional_capacity() {
    let mut map = HeaderMap::<HeaderValue>::with_capacity(5);
    map.reserve(5);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 10);
}

#[test]
fn test_reserve_with_exact_capacity() {
    let mut map = HeaderMap::<HeaderValue>::with_capacity(1);
    map.reserve(1);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 2);
}

#[test]
fn test_reserve_with_boundary_condition() {
    let mut map = HeaderMap::<HeaderValue>::with_capacity(0);
    map.reserve(1);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 2);
}

#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_reserve_panic_on_capacity_overflow() {
    let mut map = HeaderMap::<HeaderValue>::try_with_capacity(MAX_SIZE).unwrap();
    map.reserve(1); // Trying to reserve more than the max should panic.
    // Since the capacity was already at MAX_SIZE, increasing it should panic.
}

#[test]
fn test_reserve_with_large_additional_capacity() {
    let mut map = HeaderMap::<HeaderValue>::with_capacity(10);
    map.reserve(100);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 110);
}

