// Answer 0

#[test]
fn test_keys_with_multiple_entries() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    assert_eq!(map.len(), 3);
    
    let mut vec: Vec<&str> = Vec::new();
    for key in map.keys() {
        vec.push(*key);
    }
    
    vec.sort_unstable();
    assert_eq!(vec, ["a", "b", "c"]);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_keys_with_no_entries() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    
    let mut vec: Vec<&str> = Vec::new();
    for key in map.keys() {
        vec.push(*key);
    }
    
    assert!(vec.is_empty());
}

#[test]
fn test_keys_with_one_entry() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    assert_eq!(map.len(), 1);
    
    let mut vec: Vec<&str> = Vec::new();
    for key in map.keys() {
        vec.push(*key);
    }
    
    assert_eq!(vec, ["a"]);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_keys_with_identical_keys() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("a", 2); // overwriting
    assert_eq!(map.len(), 1);
    
    let mut vec: Vec<&str> = Vec::new();
    for key in map.keys() {
        vec.push(*key);
    }
    
    assert_eq!(vec, ["a"]);
    assert_eq!(map.len(), 1);
}

#[test]
fn test_keys_with_various_key_types() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    assert_eq!(map.len(), 3);
    
    let mut vec: Vec<i32> = Vec::new();
    for key in map.keys() {
        vec.push(*key);
    }
    
    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);
    assert_eq!(map.len(), 3);
}

