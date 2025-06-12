// Answer 0

#[test]
fn test_get_mut_key_not_present() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();
    // No key `2` is inserted into the map
    map.insert(1, "a");

    // Attempting to get a mutable reference to a non-existing key `2`
    let result = map.get_mut(&2);
    
    // The expected result is None, as key `2` does not exist
    assert_eq!(result, None);
}

