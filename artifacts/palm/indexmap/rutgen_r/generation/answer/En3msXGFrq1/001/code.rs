// Answer 0

#[derive(Debug, Hash, PartialEq, Eq)]
struct Key;

#[derive(Debug, PartialEq)]
struct Value;

#[test]
fn test_remove_existing_key() {
    let mut map = indexmap::IndexMap::new();
    map.insert(Key {}, Value {});

    let removed_value = map.remove(&Key {});
    assert_eq!(removed_value, Some(Value {}));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_non_existing_key() {
    let mut map = indexmap::IndexMap::new();
    map.insert(Key {}, Value {});

    let removed_value = map.remove(&Key {});
    let second_removal = map.remove(&Key {});
    
    assert_eq!(removed_value, Some(Value {}));
    assert_eq!(second_removal, None);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_key_from_empty_map() {
    let mut map = indexmap::IndexMap::new();

    let removed_value = map.remove(&Key {});
    assert_eq!(removed_value, None);
    assert_eq!(map.len(), 0);
}

#[test]
fn test_remove_with_multiple_elements() {
    let mut map = indexmap::IndexMap::new();
    map.insert(Key {}, Value {});
    map.insert(Key {}, Value {});
    
    let removed_first_value = map.remove(&Key {});
    assert!(removed_first_value.is_some());
    assert_eq!(map.len(), 1);
    
    let removed_second_value = map.remove(&Key {});
    assert!(removed_second_value.is_some());
    assert_eq!(map.len(), 0);
}

