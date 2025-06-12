// Answer 0

#[test]
fn test_len_empty() {
    let map: HashMap<i32, &str> = HashMap::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_single_element() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.len(), 1);
}

#[test]
fn test_len_multiple_elements() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    assert_eq!(map.len(), 3);
}

#[test]
fn test_len_after_clear() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.clear();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_after_removals() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    assert_eq!(map.len(), 2);
    map.remove(&1);
    assert_eq!(map.len(), 1);
    map.remove(&2);
    assert_eq!(map.len(), 0);
}

