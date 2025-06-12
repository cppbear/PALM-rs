// Answer 0

#[test]
fn test_sort_unstable_keys_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.sort_unstable_keys();
    assert!(map.is_empty());
}

#[test]
fn test_sort_unstable_keys_single_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.sort_unstable_keys();
    assert_eq!(map.get(&1), Some(&"one".to_string()));
}

#[test]
fn test_sort_unstable_keys_multiple_elements_sorted() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(2, "two".to_string());
    map.insert(1, "one".to_string());
    map.insert(3, "three".to_string());
    map.sort_unstable_keys();
    
    let keys: Vec<u32> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn test_sort_unstable_keys_multiple_elements_unsorted() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(5, "five".to_string());
    map.insert(3, "three".to_string());
    map.insert(4, "four".to_string());
    map.sort_unstable_keys();
    
    let keys: Vec<u32> = map.keys().cloned().collect();
    assert_eq!(keys, vec![3, 4, 5]);
}

#[test]
fn test_sort_unstable_keys_multiple_elements_with_duplicates() {
    use indexmap::IndexMap;

    let mut map: IndexMap<u32, String> = IndexMap::new();
    map.insert(2, "two".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "another two".to_string());
    map.sort_unstable_keys();
    
    let keys: Vec<u32> = map.keys().cloned().collect();
    assert_eq!(keys, vec![1, 2, 2]); // Duplicates may maintain the last inserted value
}

