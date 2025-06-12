// Answer 0

#[test]
fn test_partition_point_with_all_true() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.partition_point(|_k, _v| true);
    assert_eq!(result, 3); // All elements match the predicate
}

#[test]
fn test_partition_point_with_all_false() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.partition_point(|_k, _v| false);
    assert_eq!(result, 0); // No elements match the predicate
}

#[test]
fn test_partition_point_with_mixed_conditions() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    map.insert(4, "d");
    map.insert(5, "e");

    let result = map.partition_point(|k, _v| *k < 4);
    assert_eq!(result, 3); // Index of first element not matching (key 4)
}

#[test]
fn test_partition_point_at_boundary() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.partition_point(|k, _v| *k <= 2);
    assert_eq!(result, 2); // Boundary case, includes key 2
}

#[test]
#[should_panic(expected = "Index out of bounds")] // Assuming the case where the internal structure may not handle an edge case
fn test_partition_point_with_empty_map() {
    let map: IndexMap<i32, &str> = IndexMap::new();
    let _ = map.partition_point(|_k, _v| false); // Should not panic, but a safeguard for potential panic in implementation
}

