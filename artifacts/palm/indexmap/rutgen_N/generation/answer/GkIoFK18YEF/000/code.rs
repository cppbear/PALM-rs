// Answer 0

#[test]
fn test_binary_search_by_found() {
    use std::cmp::Ordering;
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let result = map.binary_search_by(|key, _| {
        if key == &&"b" {
            Ordering::Equal
        } else if key < &&"b" {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_not_found() {
    use std::cmp::Ordering;
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    let result = map.binary_search_by(|key, _| {
        if key == &&"d" {
            Ordering::Equal
        } else if key < &&"d" {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_insert_position() {
    use std::cmp::Ordering;
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("c", 3);
    
    let result = map.binary_search_by(|key, _| {
        if key == &&"b" {
            Ordering::Equal
        } else if key < &&"b" {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Err(1));
}

