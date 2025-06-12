// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(0);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_non_zero() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(10);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
    map.reserve(5);
    assert!(map.capacity() >= 5);
}

#[test]
fn test_with_capacity_boundary() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(usize::MAX);
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

