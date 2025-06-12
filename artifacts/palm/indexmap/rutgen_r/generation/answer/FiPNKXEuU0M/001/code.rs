// Answer 0

#[test]
fn test_sort_by_cached_key_integers() {
    let mut map = indexmap::IndexMap::new();
    map.insert("a", 3);
    map.insert("b", 1);
    map.insert("c", 2);
    
    map.sort_by_cached_key(|&value| value);
    
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["b", "c", "a"]);
}

#[test]
fn test_sort_by_cached_key_floats() {
    let mut map = indexmap::IndexMap::new();
    map.insert("x", 2.5);
    map.insert("y", 1.0);
    map.insert("z", 3.4);
    
    map.sort_by_cached_key(|&value| value);
    
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["y", "x", "z"]);
}

#[test]
fn test_sort_by_cached_key_strings() {
    let mut map = indexmap::IndexMap::new();
    map.insert("apple", "banana");
    map.insert("cherry", "apple");
    map.insert("date", "cherry");

    map.sort_by_cached_key(|&value| value);
    
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["apple", "date", "cherry"]);
}

#[test]
fn test_sort_by_cached_key_empty() {
    let mut map: indexmap::IndexMap<&str, i32> = indexmap::IndexMap::new();
    
    map.sort_by_cached_key(|&value| value); // should not panic
    
    assert!(map.is_empty());
}

#[test]
fn test_sort_by_cached_key_single_entry() {
    let mut map = indexmap::IndexMap::new();
    map.insert("only", 42);
    
    map.sort_by_cached_key(|&value| value);
    
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["only"]);
}

