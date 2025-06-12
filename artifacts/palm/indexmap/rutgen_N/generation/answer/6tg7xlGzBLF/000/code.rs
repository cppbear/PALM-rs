// Answer 0

#[test]
fn test_sort_keys_empty() {
    use indexmap::IndexMap; // Adjust import as necessary in your context
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.sort_keys();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_sort_keys_single_element() {
    use indexmap::IndexMap; // Adjust import as necessary in your context
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(5, 10);
    map.sort_keys();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&5), Some(&10));
}

#[test]
fn test_sort_keys_two_elements_unsorted() {
    use indexmap::IndexMap; // Adjust import as necessary in your context
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.sort_keys();
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2]);
}

#[test]
fn test_sort_keys_two_elements_sorted() {
    use indexmap::IndexMap; // Adjust import as necessary in your context
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.sort_keys();
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2]);
}

#[test]
fn test_sort_keys_with_duplicates() {
    use indexmap::IndexMap; // Adjust import as necessary in your context
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.insert(2, 30); // Same key, different value
    map.sort_keys();
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2]);
    assert_eq!(map.get(&2), Some(&30)); // Check if the last value for key 2 is retained
}

