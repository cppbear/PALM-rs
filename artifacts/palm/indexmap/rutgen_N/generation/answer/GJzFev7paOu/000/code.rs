// Answer 0

#[test]
fn test_drain_empty_indexmap() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    let drained: Vec<_> = map.drain(..).collect();
    assert!(drained.is_empty());
}

#[test]
fn test_drain_full_indexmap() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    let drained: Vec<_> = map.drain(..).collect();
    assert_eq!(drained.len(), 3);
    assert!(map.is_empty());
    assert!(drained.contains(&(1, "one".to_string())));
    assert!(drained.contains(&(2, "two".to_string())));
    assert!(drained.contains(&(3, "three".to_string())));
}

#[test]
fn test_drain_partial_range() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    let drained: Vec<_> = map.drain(1..3).collect();
    assert_eq!(drained.len(), 2);
    assert!(map.len() == 1);
    assert!(map.contains_key(&1));
    assert!(!map.contains_key(&2));
    assert!(!map.contains_key(&3));
}

#[test]
#[should_panic]
fn test_drain_panic_start_greater_than_end() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    let _drained: Vec<_> = map.drain(2..1).collect();
}

#[test]
#[should_panic]
fn test_drain_panic_end_greater_than_length() {
    let mut map: indexmap::IndexMap<u32, String> = indexmap::IndexMap::new();
    map.insert(1, "one".to_string());

    let _drained: Vec<_> = map.drain(..2).collect();
}

