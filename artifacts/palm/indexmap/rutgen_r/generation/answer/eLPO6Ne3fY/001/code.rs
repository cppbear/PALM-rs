// Answer 0

#[test]
fn test_shrink_to_fit_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.shrink_to_fit();
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_shrink_to_fit_single_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.shrink_to_fit();
    assert_eq!(map.capacity(), 1);
}

#[test]
fn test_shrink_to_fit_multiple_elements() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.shrink_to_fit();
    assert_eq!(map.capacity(), 3);
}

#[test]
fn test_shrink_to_fit_with_removals() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.remove(&1);
    map.shrink_to_fit();
    assert_eq!(map.capacity(), 1);
}

