// Answer 0

#[test]
fn test_sort_by_empty_map() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.sort_by(|_k1, _v1, _k2, _v2| std::cmp::Ordering::Equal);
    assert!(map.is_empty());
}

#[test]
fn test_sort_by_single_element() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.sort_by(|_k1, _v1, _k2, _v2| std::cmp::Ordering::Equal);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&10));
}

#[test]
fn test_sort_by_two_elements() {
    let mut map = indexmap::IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.sort_by(|k1, v1, k2, v2| k1.cmp(k2));
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2]);
}

#[test]
fn test_sort_by_with_same_keys() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 20);
    map.insert(1, 10);
    map.sort_by(|k1, v1, k2, v2| k1.cmp(k2));
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values, vec![10]); // Overwrites should keep the latest
}

#[test]
fn test_sort_by_reverse_order() {
    let mut map = indexmap::IndexMap::new();
    map.insert(3, 30);
    map.insert(1, 10);
    map.insert(2, 20);
    map.sort_by(|k1, _v1, k2, _v2| k2.cmp(k1)); // Sorting in descending order
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec![3, 2, 1]);
}

#[test]
fn test_sort_by_custom_value_order() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 30);
    map.insert(2, 20);
    map.insert(3, 10);
    map.sort_by(|_k1, v1, _k2, v2| v1.cmp(v2)); // Sorting by value
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values, vec![10, 20, 30]);
}

