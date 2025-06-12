// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let result = map.swap_remove_index(1);
    
    assert_eq!(result, Some((2, "two")));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&3), Some(&"three"));
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    
    let result = map.swap_remove_index(5);
    
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_swap_remove_index_empty() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    
    let result = map.swap_remove_index(0);
    
    assert_eq!(result, None);
    assert_eq!(map.len(), 0);
}

