// Answer 0

#[test]
fn test_values_mut_with_non_empty_map() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    let mut values_iter = map.values_mut();
    assert_eq!(values_iter.next(), Some(&mut 1));
    assert_eq!(values_iter.next(), Some(&mut 2));
    assert_eq!(values_iter.next(), None);
}

#[test]
fn test_values_mut_with_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<String, i32> = IndexMap::new();
    let mut values_iter = map.values_mut();
    assert_eq!(values_iter.next(), None);
}

#[test]
fn test_values_mut_reflected_in_map() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    {
        let values_iter = map.values_mut();
        if let Some(value) = values_iter.next() {
            *value += 10;
        }
    }

    assert_eq!(map.get("key1"), Some(&11));
    assert_eq!(map.get("key2"), Some(&2));
}

#[test]
#[should_panic]
fn test_values_mut_panic_on_borrowing() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    let mut values_iter = map.values_mut();
    let first_value = values_iter.next().unwrap();
    let second_value = values_iter.next().unwrap();
    
    *first_value += 10;
    assert_eq!(*second_value, 2); // This accesses second_value while first_value is still in scope, leading to a panic.
}

