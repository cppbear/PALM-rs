// Answer 0

#[test]
fn test_shrink_to_valid_capacity() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let original_capacity = map.len();

    map.shrink_to(2);
    assert!(map.len() == 2);
    assert!(map.get(&1).is_some());
    assert!(map.get(&2).is_some());
    assert!(map.get(&3).is_none());
}

#[test]
fn test_shrink_to_no_change() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let original_capacity = map.len();

    map.shrink_to(2);
    assert!(map.len() == original_capacity);
}

#[test]
fn test_shrink_to_zero_capacity() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);

    map.shrink_to(0);
    assert!(map.is_empty());
}

#[test]
#[should_panic]
fn test_shrink_to_excessive_capacity() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    
    map.shrink_to(10); // This should panic because min_capacity exceeds the current size.
}

