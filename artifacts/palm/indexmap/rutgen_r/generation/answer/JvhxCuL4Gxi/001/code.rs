// Answer 0

#[test]
fn test_reserve_exact_no_additional() {
    let mut index_map = IndexMap::new();
    index_map.reserve_exact(0);
    assert_eq!(index_map.entries.len(), 0);
}

#[test]
fn test_reserve_exact_small_capacity() {
    let mut index_map = IndexMap::new();
    index_map.entries.push((1, "one"));
    index_map.reserve_exact(1);
    assert!(index_map.entries.capacity() >= 2);
}

#[test]
fn test_reserve_exact_large_capacity() {
    let mut index_map = IndexMap::new();
    index_map.entries.push((1, "one"));
    index_map.entries.push((2, "two"));
    index_map.reserve_exact(10);
    assert!(index_map.entries.capacity() >= 12);
}

#[test]
fn test_reserve_exact_exceeding_existing_capacity() {
    let mut index_map = IndexMap::with_capacity(5);
    index_map.reserve_exact(10);
    assert!(index_map.entries.capacity() >= 15);
}

#[test]
#[should_panic]
fn test_reserve_exact_negative() {
    let mut index_map = IndexMap::new();
    index_map.reserve_exact(usize::MAX);
}

