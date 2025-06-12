// Answer 0

#[test]
fn test_pop_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let result = map.pop();
    assert_eq!(result, None);
}

#[test]
fn test_pop_single_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    let result = map.pop();
    assert_eq!(result, Some((1, 100)));
    assert!(map.is_empty());
}

#[test]
fn test_pop_multiple_elements() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    let result = map.pop();
    assert_eq!(result, Some((3, 300)));
    assert_eq!(map.len(), 2);
    
    let result = map.pop();
    assert_eq!(result, Some((2, 200)));
    assert_eq!(map.len(), 1);
    
    let result = map.pop();
    assert_eq!(result, Some((1, 100)));
    assert!(map.is_empty());
}

