// Answer 0

#[test]
fn test_get_key_value_mut_existing_key() {
    use hashbrown::HashMap;
    let mut map = HashMap::new();
    map.insert(1, "value1");
    
    // Test for existing key
    let (k, v) = map.get_key_value_mut(&1).unwrap();
    assert_eq!(k, &1);
    assert_eq!(v, &mut "value1");
    
    // Modify the value through mutable reference
    *v = "modified_value1";
    assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "modified_value1")));
}

#[test]
fn test_get_key_value_mut_non_existing_key() {
    use hashbrown::HashMap;
    let mut map = HashMap::new();
    map.insert(1, "value1");

    // Test for non-existing key
    let result = map.get_key_value_mut(&2);
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_mut_multiple_insertions() {
    use hashbrown::HashMap;
    let mut map = HashMap::new();
    map.insert(1, "value1");
    map.insert(2, "value2");

    // Test for an existing key
    let (k, v) = map.get_key_value_mut(&1).unwrap();
    assert_eq!(k, &1);
    assert_eq!(v, &mut "value1");

    // Modify the value through the mutable reference
    *v = "modified_value1";
    assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "modified_value1")));
    
    // Ensure other keys remain unaffected
    let (k2, v2) = map.get_key_value_mut(&2).unwrap();
    assert_eq!(k2, &2);
    assert_eq!(v2, &mut "value2");
}

