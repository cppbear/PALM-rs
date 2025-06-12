// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: indexmap::IndexMap<_, _> = indexmap::IndexMap::with_capacity(0);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_non_zero() {
    let capacity = 10;
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(capacity);
    assert_eq!(map.capacity(), capacity);
}

#[test]
fn test_with_capacity_large_number() {
    let capacity = 1_000_000;
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(capacity);
    assert_eq!(map.capacity(), capacity);
}

#[test]
fn test_with_capacity_uint_max() {
    let capacity = std::usize::MAX;
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(capacity);
    assert_eq!(map.capacity(), capacity);
}

