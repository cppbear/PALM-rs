// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    let removed = map.shift_remove_index(1);
    assert_eq!(removed, Some(("key2", 2)));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&1));
    assert_eq!(map.get("key3"), Some(&3));
}

#[test]
fn test_shift_remove_index_first() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    let removed = map.shift_remove_index(0);
    assert_eq!(removed, Some(("key1", 1)));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key2"), Some(&2));
}

#[test]
fn test_shift_remove_index_last() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    let removed = map.shift_remove_index(2);
    assert_eq!(removed, Some(("key3", 3)));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&1));
    assert_eq!(map.get("key2"), Some(&2));
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    let removed = map.shift_remove_index(2);
    assert_eq!(removed, None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_shift_remove_index_empty() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    
    let removed = map.shift_remove_index(0);
    assert_eq!(removed, None);
    assert_eq!(map.len(), 0);
}

