// Answer 0

#[test]
fn test_keys_basic() {
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
fn test_keys_empty() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    let keys: Vec<&str> = map.keys().collect();
    assert_eq!(keys.len(), 0);
}

#[test]
fn test_keys_single_entry() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    
    assert_eq!(map.len(), 1);
    let mut vec: Vec<&str> = Vec::new();

    for key in map.keys() {
        vec.push(*key);
    }

    vec.sort_unstable();
    assert_eq!(vec, ["a"]);
    assert_eq!(map.len(), 1);
}

