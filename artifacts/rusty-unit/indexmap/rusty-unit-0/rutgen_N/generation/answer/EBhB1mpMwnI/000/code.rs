// Answer 0

#[test]
fn test_reserve_exact_with_zero_additional() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.reserve_exact(0);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_reserve_exact_with_positive_additional() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.reserve_exact(1);
    assert!(map.capacity() >= 2); // At least 2 due to 1 existing key-value pair and 1 reserved
}

#[test]
fn test_reserve_exact_increases_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.reserve_exact(10);
    let initial_capacity = map.capacity();
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    assert!(map.capacity() > initial_capacity);
}

#[test]
#[should_panic]
fn test_reserve_exact_exceeds_max_capacity() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let max_additional = std::usize::MAX; // Simulating a capacity overflow
    map.reserve_exact(max_additional);
}

