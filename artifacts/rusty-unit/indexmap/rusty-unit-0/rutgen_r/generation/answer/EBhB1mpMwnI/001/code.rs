// Answer 0

#[test]
fn test_reserve_exact_zero_additional() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.reserve_exact(0);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_reserve_exact_small_additional() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.reserve_exact(2);
    assert!(map.capacity() >= 4); // Ensure the capacity is enough for additional pairs
}

#[test]
fn test_reserve_exact_large_additional() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.reserve_exact(1000);
    assert!(map.capacity() >= 1000); // Ensure we reserved space for large additional
}

#[should_panic]
fn test_reserve_exact_panic() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let _ = map.reserve_exact(usize::MAX); // This should panic due to overflow in allocation
}

