// Answer 0

#[test]
fn test_iter_mut_empty_map() {
    use indexmap::IndexMap; // Assuming that's where your map implementation is from

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let mut iter = map.iter_mut();

    assert!(iter.next().is_none());
}

#[test]
fn test_iter_mut_single_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    let mut iter = map.iter_mut();

    if let Some((key, value)) = iter.next() {
        assert_eq!(*key, 1);
        *value += 50;
    } else {
        panic!("Expected a value but found none");
    }

    assert_eq!(map[&1], 150);
}

#[test]
fn test_iter_mut_multiple_elements() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    let mut iter = map.iter_mut();

    while let Some((key, value)) = iter.next() {
        *value += 10;
    }

    assert_eq!(map[&1], 110);
    assert_eq!(map[&2], 210);
    assert_eq!(map[&3], 310);
}

#[test]
#[should_panic]
fn test_iter_mut_panic_on_invalid_access() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);

    let mut iter = map.iter_mut();
    let value = iter.next().expect("Expected a value").1;

    // Simulate invalid access by calling iter.next() again after modifying the map
    map.remove(&1);
    let _ = iter.next(); // Attempting to access after the map has been mutated
}

