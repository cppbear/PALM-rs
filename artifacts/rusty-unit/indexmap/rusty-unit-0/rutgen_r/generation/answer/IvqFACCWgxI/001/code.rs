// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(0);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_one() {
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(1);
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 1);
}

#[test]
fn test_with_capacity_large() {
    let n: usize = 1_000_000; // Large capacity for performance testing
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(n);
    assert!(map.is_empty());
    assert_eq!(map.capacity(), n);
}

#[test]
fn test_with_capacity_boundary_small() {
    let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(2);
    assert!(map.is_empty());
    assert_eq!(map.capacity(), 2);
}

#[test]
fn test_with_capacity_boundary_large() {
    let n: usize = std::usize::MAX; // Testing the maximum size
    let map_result = std::panic::catch_unwind(|| {
        let map: indexmap::IndexMap<usize, usize> = indexmap::IndexMap::with_capacity(n);
        map
    });
    assert!(map_result.is_err()); // Expecting a panic due to maximum capacity
}

