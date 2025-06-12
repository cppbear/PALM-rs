// Answer 0

#[test]
fn test_shift_remove_full_empty_map() {
    use std::collections::HashMap;
    use std::hash::Hash;

    // Define the key-value types
    let mut map: HashMap<i32, String> = HashMap::new();
    
    // Attempt to remove a key from an empty map
    let result = map.shift_remove_full(&1);
    assert!(result.is_none());
}

#[test]
fn test_shift_remove_full_single_element_no_match() {
    use std::collections::HashMap;
    use std::hash::Hash;

    // Define the key-value types
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("value1"));

    // Attempt to remove a key that does not match the single element
    let result = map.shift_remove_full(&2);
    assert!(result.is_none());
}

#[test]
fn test_shift_remove_full_single_element_match() {
    use std::collections::HashMap;
    use std::hash::Hash;

    // Define the key-value types
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("value1"));

    // Remove the existing key
    let result = map.shift_remove_full(&1);
    assert_eq!(result, Some((0, 1, String::from("value1"))));
}

#[test]
fn test_shift_remove_full_multiple_elements() {
    use std::collections::HashMap;
    use std::hash::Hash;

    // Define the key-value types
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("value1"));
    map.insert(2, String::from("value2"));
    map.insert(3, String::from("value3"));

    // Attempt to remove a key that does not match the first element
    let result = map.shift_remove_full(&4);
    assert!(result.is_none());
}

#[test]
fn test_shift_remove_full_multiple_elements_first_match() {
    use std::collections::HashMap;
    use std::hash::Hash;

    // Define the key-value types
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("value1"));
    map.insert(2, String::from("value2"));
    map.insert(3, String::from("value3"));

    // Remove the first element
    let result = map.shift_remove_full(&1);
    assert_eq!(result, Some((0, 1, String::from("value1"))));
}

