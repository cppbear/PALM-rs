// Answer 0

#[test]
fn test_binary_search_by_found() {
    let map = indexmap::IndexMap::from_iter(vec![(1, "a"), (2, "b"), (3, "c")]);

    let result = map.binary_search_by(|&k, _| k.cmp(&2));
    
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_not_found() {
    let map = indexmap::IndexMap::from_iter(vec![(1, "a"), (2, "b"), (3, "c")]);

    let result = map.binary_search_by(|&k, _| k.cmp(&4));
    
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_first_element() {
    let map = indexmap::IndexMap::from_iter(vec![(1, "a"), (2, "b"), (3, "c")]);

    let result = map.binary_search_by(|&k, _| k.cmp(&1));
    
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_last_element() {
    let map = indexmap::IndexMap::from_iter(vec![(1, "a"), (2, "b"), (3, "c")]);

    let result = map.binary_search_by(|&k, _| k.cmp(&3));
    
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_empty_map() {
    let map: indexmap::IndexMap<i32, &str> = indexmap::IndexMap::new();

    let result = map.binary_search_by(|&k, _| k.cmp(&1));
    
    assert_eq!(result, Err(0));
}

