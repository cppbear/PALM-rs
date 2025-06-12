// Answer 0

#[test]
fn test_values_mut() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    for val in map.values_mut() {
        *val += 10;
    }

    assert_eq!(map.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    vec.sort_unstable();
    assert_eq!(vec, [11, 12, 13]);

    assert_eq!(map.len(), 3);
}

#[test]
fn test_values_mut_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, i32> = HashMap::new();

    for val in map.values_mut() {
        *val += 10; // This should not panic or change anything as the map is empty
    }

    assert_eq!(map.len(), 0);
}

#[test]
fn test_values_mut_boundary_conditions() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("x", 1);

    // Only one element in the map
    for val in map.values_mut() {
        *val += 5;
    }

    assert_eq!(map.len(), 1);
    let mut vec: Vec<i32> = Vec::new();

    for val in map.values() {
        vec.push(*val);
    }

    vec.sort_unstable();
    assert_eq!(vec, [6]); // Ensure the value was updated correctly
}

