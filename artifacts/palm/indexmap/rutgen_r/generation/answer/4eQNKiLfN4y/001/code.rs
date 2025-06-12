// Answer 0

#[test]
fn test_capacity_empty_map() {
    let map: indexmap::IndexMap<u32, u32> = indexmap::IndexMap::new();
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_non_empty_map() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    assert!(map.capacity() >= 2); // Should be at least capacity for 2 elements
}

#[test]
fn test_capacity_after_reserve() {
    let mut map = indexmap::IndexMap::new();
    map.reserve(10);
    assert!(map.capacity() >= 10); // Should at least be able to hold 10 elements
}

#[test]
fn test_capacity_large_map() {
    let mut map = indexmap::IndexMap::new();
    for i in 0..100 {
        map.insert(i, i);
    }
    assert!(map.capacity() >= 100); // Should be able to hold at least 100 elements
}

#[test]
fn test_capacity_with_filled_map() {
    let mut map = indexmap::IndexMap::new();
    for i in 0..50 {
        map.insert(i, i);
    }
    let current_capacity = map.capacity();
    assert!(current_capacity >= 50); // Ensure it can hold at least 50
    map.insert(51, 51);
    assert!(map.capacity() >= 51); // Ensure capacity may increase
}

