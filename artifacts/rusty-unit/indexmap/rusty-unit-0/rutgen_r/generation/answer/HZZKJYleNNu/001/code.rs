// Answer 0

#[test]
fn test_iter_mut2_with_non_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "Value1".to_string());
    map.insert(2, "Value2".to_string());

    let mut iter = map.iter_mut2();
    assert_eq!(iter.next().unwrap(), (&1, &mut "Value1".to_string()));
    assert_eq!(iter.next().unwrap(), (&2, &mut "Value2".to_string()));
}

#[test]
fn test_iter_mut2_with_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, String> = IndexMap::new();

    let iter = map.iter_mut2();
    assert!(iter.next().is_none());
}

#[should_panic]
fn test_iter_mut2_on_frozen_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "Value1".to_string());

    // Assuming this simulates a frozen state. In practice, the state would need to be
    // defined such that mutation is impossible.
    let _frozen = &map; 
    let mut iter = _frozen.iter_mut2();  // This would cause a panic theoretically
    let _ = iter.next(); // Prevent unused variable warning
}

