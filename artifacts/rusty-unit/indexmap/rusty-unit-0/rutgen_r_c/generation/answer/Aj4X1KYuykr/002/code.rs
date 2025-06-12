// Answer 0

#[test]
fn test_insert_sorted_updates_existing_key() {
    #[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);

    let mut map = IndexMap::new();
    map.insert(KeyType(1), ValueType(10));
    map.insert(KeyType(2), ValueType(20));

    let (index, old_value) = map.insert_sorted(KeyType(1), ValueType(30));

    assert_eq!(index, 0);
    assert_eq!(old_value, Some(ValueType(10)));
    assert_eq!(map.get_index(index), Some((&KeyType(1), &mut ValueType(30))));
}

#[test]
fn test_insert_sorted_inserts_new_key() {
    #[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
    struct KeyType(i32);

    #[derive(Debug)]
    struct ValueType(i32);

    let mut map = IndexMap::new();
    map.insert(KeyType(1), ValueType(10));
    map.insert(KeyType(3), ValueType(30));

    let (index, old_value) = map.insert_sorted(KeyType(2), ValueType(20));

    assert_eq!(index, 1);
    assert_eq!(old_value, None);
    assert_eq!(map.get_index(index), Some((&KeyType(2), &ValueType(20))));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_insert_sorted_panics_out_of_bounds() {
    #[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);

    let mut map = IndexMap::new();
    
    // This will panic since we are accessing an index that does not exist yet.
    let _ = map.insert_sorted(KeyType(99), ValueType(100));
}

#[test]
fn test_insert_sorted_multiple_updates() {
    #[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);

    let mut map = IndexMap::new();
    map.insert(KeyType(1), ValueType(10));
    map.insert(KeyType(2), ValueType(20));
    map.insert(KeyType(3), ValueType(30));
    
    let (index1, old_value1) = map.insert_sorted(KeyType(2), ValueType(40));
    let (index2, old_value2) = map.insert_sorted(KeyType(3), ValueType(50));

    assert_eq!(index1, 1);
    assert_eq!(old_value1, Some(ValueType(20)));
    assert_eq!(index2, 2);
    assert_eq!(old_value2, Some(ValueType(30)));
}

