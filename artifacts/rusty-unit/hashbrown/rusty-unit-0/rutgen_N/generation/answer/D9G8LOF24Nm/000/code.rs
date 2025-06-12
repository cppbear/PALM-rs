// Answer 0

#[test]
fn test_values_non_empty() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    assert_eq!(map.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);

    assert_eq!(map.len(), 3);
}

#[test]
fn test_values_empty() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    
    assert_eq!(map.len(), 0);
    let vec: Vec<i32> = map.values().cloned().collect();

    assert!(vec.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_values_with_duplicates() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 1);
    map.insert("c", 2);
    
    assert_eq!(map.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    vec.sort_unstable();
    assert_eq!(vec, [1, 1, 2]);

    assert_eq!(map.len(), 3);
}

