// Answer 0

#[test]
fn test_sort_keys_empty_map() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.sort_keys();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_sort_keys_single_element() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, "a");
    map.sort_keys();
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![1]);
}

#[test]
fn test_sort_keys_multiple_elements_sorted() {
    let mut map = indexmap::IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    map.sort_keys();
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_sort_keys_multiple_elements_reverse_order() {
    let mut map = indexmap::IndexMap::new();
    map.insert(3, "c");
    map.insert(2, "b");
    map.insert(1, "a");
    map.sort_keys();
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
}

#[test]
fn test_sort_keys_multiple_elements_with_duplicates() {
    let mut map = indexmap::IndexMap::new();
    map.insert(2, "b");
    map.insert(1, "a");
    map.insert(2, "c"); // duplicate key, should keep first insertion
    map.sort_keys();
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![1, 2]);
}

#[test]
#[should_panic]
fn test_sort_keys_with_non_ord_key() {
    struct UnorderedStruct;
    let mut map: indexmap::IndexMap<UnorderedStruct, i32> = indexmap::IndexMap::new();
    map.insert(UnorderedStruct, 1);
    map.sort_keys(); // This should panic
}

