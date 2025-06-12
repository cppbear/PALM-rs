// Answer 0

#[test]
fn test_pop_from_empty_indexmap() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<i32, ()> = IndexMap::new();
    let result = map.pop();
    assert_eq!(result, None);
}

#[test]
fn test_pop_single_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, ()> = IndexMap::new();
    map.insert(1, ());
    
    let result = map.pop();
    assert_eq!(result, Some(1));
    assert_eq!(map.len(), 0);
}

#[test]
fn test_pop_multiple_elements() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, ()> = IndexMap::new();
    map.insert(1, ());
    map.insert(2, ());
    map.insert(3, ());
    
    let result = map.pop();
    assert_eq!(result, Some(3));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_pop_preserves_order() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, ()> = IndexMap::new();
    map.insert(1, ());
    map.insert(2, ());
    map.insert(3, ());
    
    let _ = map.pop(); // pop last element
    let result = map.pop(); // now pop the new last element
    assert_eq!(result, Some(2));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_multiple_pops() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, ()> = IndexMap::new();
    map.insert(1, ());
    map.insert(2, ());
    map.insert(3, ());
    
    let _ = map.pop(); // pop 3
    let _ = map.pop(); // pop 2
    let result = map.pop(); // should pop 1
    assert_eq!(result, Some(1));
    assert_eq!(map.len(), 0);
}

