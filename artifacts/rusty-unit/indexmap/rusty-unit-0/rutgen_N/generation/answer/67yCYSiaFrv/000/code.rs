// Answer 0

#[test]
fn test_get_full_existing_key() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    let result = map.get_full(&"a");
    assert_eq!(result, Some((0, &"a", &1)));
}

#[test]
fn test_get_full_non_existing_key() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    let result = map.get_full(&"c");
    assert_eq!(result, None);
}

#[test]
fn test_get_full_empty_map() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    let map: IndexMap<&str, i32> = IndexMap::new();
    
    let result = map.get_full(&"a");
    assert_eq!(result, None);
}

