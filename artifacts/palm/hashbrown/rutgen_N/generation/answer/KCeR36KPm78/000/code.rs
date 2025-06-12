// Answer 0

#[test]
fn test_iter_mut_update_values() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Update all values
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }

    assert_eq!(map.len(), 3);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in &map {
        vec.push((*key, *val));
    }

    vec.sort_unstable();
    assert_eq!(vec, [("a", 2), ("b", 4), ("c", 6)]);
}

#[test]
fn test_iter_mut_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    
    // Iterate over an empty map
    for (_, val) in map.iter_mut() {
        // This block should not execute
        panic!("Iterator should not yield any items for an empty map");
    }

    assert_eq!(map.len(), 0);
}

#[test]
fn test_iter_mut_single_entry() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 10);

    // Update value
    for (_, val) in map.iter_mut() {
        *val += 5;
    }

    assert_eq!(map.len(), 1);
    let mut vec: Vec<(&str, i32)> = Vec::new();

    for (key, val) in &map {
        vec.push((*key, *val));
    }
    
    vec.sort_unstable();
    assert_eq!(vec, [("a", 15)]);
}

