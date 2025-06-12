// Answer 0

#[test]
fn test_is_empty_on_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    assert!(!map.is_empty());
}

#[test]
fn test_is_empty_after_removing_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.remove(&1);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_on_large_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, i * 10);
    }
    assert!(!map.is_empty());
    
    // Clear the map to ensure it's empty
    map.clear();
    assert!(map.is_empty());
}

