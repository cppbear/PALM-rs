// Answer 0

#[test]
fn test_iter_mut_basic_functionality() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Update all values
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }

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

    // Iterating over an empty map
    let mut vec: Vec<(&str, i32)> = Vec::new();
    for _ in map.iter_mut() {
        panic!("Should not enter the loop for an empty map");
    }
    assert!(vec.is_empty());
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

    let mut vec: Vec<(&str, i32)> = Vec::new();
    for (key, val) in &map {
        vec.push((*key, *val));
    }

    vec.sort_unstable();
    assert_eq!(vec, [("a", 15)]);
}

#[test]
#[should_panic(expected = "Should not enter the loop for an empty map")]
fn test_iter_mut_panic_condition() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();

    // Attempting to iterate and trigger panic in empty state
    for (_, _) in map.iter_mut() {
        panic!("Should not enter the loop for an empty map");
    }
}

