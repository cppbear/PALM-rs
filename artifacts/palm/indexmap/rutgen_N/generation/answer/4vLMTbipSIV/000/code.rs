// Answer 0

#[test]
fn test_iter_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();
    let iter = map.iter();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, Vec::<(&i32, &i32)>::new());
}

#[test]
fn test_iter_single_element() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 2);
    let iter = map.iter();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![(&1, &2)]);
}

#[test]
fn test_iter_multiple_elements() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map.insert(5, 6);
    let iter = map.iter();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![(&1, &2), (&3, &4), (&5, &6)]);
}

