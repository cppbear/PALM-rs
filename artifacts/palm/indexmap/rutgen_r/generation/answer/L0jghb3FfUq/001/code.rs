// Answer 0

#[test]
fn test_sort_unstable_keys_empty() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.sort_unstable_keys();
    assert!(map.is_empty());
}

#[test]
fn test_sort_unstable_keys_single_element() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.sort_unstable_keys();
    assert_eq!(map.keys().cloned().collect::<Vec<_>>(), vec![1]);
    assert_eq!(map.values().cloned().collect::<Vec<_>>(), vec![10]);
}

#[test]
fn test_sort_unstable_keys_two_unique_elements() {
    let mut map = indexmap::IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.sort_unstable_keys();
    assert_eq!(map.keys().cloned().collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(map.values().cloned().collect::<Vec<_>>(), vec![10, 20]);
}

#[test]
fn test_sort_unstable_keys_two_equal_elements() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.insert(1, 20); // This will overwrite the previous one
    map.sort_unstable_keys();
    assert_eq!(map.keys().cloned().collect::<Vec<_>>(), vec![1]);
    assert_eq!(map.values().cloned().collect::<Vec<_>>(), vec![20]);
}

#[test]
fn test_sort_unstable_keys_multiple_elements() {
    let mut map = indexmap::IndexMap::new();
    map.insert(3, 30);
    map.insert(1, 10);
    map.insert(2, 20);
    map.sort_unstable_keys();
    assert_eq!(map.keys().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(map.values().cloned().collect::<Vec<_>>(), vec![10, 20, 30]);
}

#[test]
fn test_sort_unstable_keys_with_negative_and_positive() {
    let mut map = indexmap::IndexMap::new();
    map.insert(-1, 10);
    map.insert(0, 20);
    map.insert(1, 30);
    map.sort_unstable_keys();
    assert_eq!(map.keys().cloned().collect::<Vec<_>>(), vec![-1, 0, 1]);
    assert_eq!(map.values().cloned().collect::<Vec<_>>(), vec![10, 20, 30]);
}

