// Answer 0

#[test]
fn test_into_values() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let mut vec: Vec<i32> = map.into_values().collect();
    
    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn test_empty_into_values() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    let vec: Vec<i32> = map.into_values().collect();
    
    assert_eq!(vec, Vec::<i32>::new());
}

#[test]
fn test_single_value_into_values() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 10);

    let mut vec: Vec<i32> = map.into_values().collect();
    
    vec.sort_unstable();
    assert_eq!(vec, [10]);
}

