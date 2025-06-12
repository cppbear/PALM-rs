// Answer 0

#[test]
fn test_sort_unstable_keys_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_sort_unstable_keys_single_element() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[(1, 10)]);
}

#[test]
fn test_sort_unstable_keys_sorted() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[(1, 10), (2, 20), (3, 30)]);
}

#[test]
fn test_sort_unstable_keys_reverse_sorted() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(3, 30);
    map.insert(2, 20);
    map.insert(1, 10);
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[(1, 10), (2, 20), (3, 30)]);
}

#[test]
fn test_sort_unstable_keys_with_duplicates() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.insert(2, 25);
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[(1, 10), (2, 20), (2, 25)]);
}

#[test]
fn test_sort_unstable_keys_equal_elements() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(1, 20);
    map.sort_unstable_keys();
    assert_eq!(map.as_slice(), &[(1, 10), (1, 20)]);
}

