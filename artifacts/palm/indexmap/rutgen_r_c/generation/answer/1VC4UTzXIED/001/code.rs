// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    let mut map: crate::IndexMap<i32, &str> = crate::IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    
    let result = map.binary_search_by_key(&2, |&k, _| k);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_not_found() {
    let mut map: crate::IndexMap<i32, &str> = crate::IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let result = map.binary_search_by_key(&4, |&k, _| k);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_key_empty_map() {
    let map: crate::IndexMap<i32, &str> = crate::IndexMap::new();
    
    let result = map.binary_search_by_key(&1, |&k, _| k);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_key_edge_case_low() {
    let mut map: crate::IndexMap<i32, &str> = crate::IndexMap::new();
    map.insert(1, "a");
  
    let result = map.binary_search_by_key(&0, |&k, _| k);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_key_edge_case_high() {
    let mut map: crate::IndexMap<i32, &str> = crate::IndexMap::new();
    map.insert(1, "a");
  
    let result = map.binary_search_by_key(&2, |&k, _| k);
    assert_eq!(result, Err(1));
}

