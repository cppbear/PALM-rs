// Answer 0

#[test]
fn test_try_insert_success() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    let result = map.try_insert(1, "test_value");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &mut "test_value");
}

#[test]
fn test_try_insert_multiple_entries() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    let result1 = map.try_insert(2, "value_1");
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), &mut "value_1");

    let result2 = map.try_insert(3, "value_2");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), &mut "value_2");

    let result3 = map.try_insert(2, "value_3"); // should not succeed, key 2 occupied
    assert!(result3.is_err());
} 

#[test]
fn test_try_insert_different_types() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    let result = map.try_insert(4, 100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &mut 100);
}

#[test]
fn test_try_insert_empty_map() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    let result = map.try_insert(5, "initial_value");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &mut "initial_value");
} 

#[test]
fn test_try_insert_multiple_same_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::OccupiedError;

    let mut map = HashMap::new();
    let _ = map.try_insert(6, "first_value");
    
    let result = map.try_insert(6, "second_value");
    assert!(result.is_err());
    if let Err(OccupiedError { entry, value }) = result {
        assert_eq!(entry.key(), &6);
        assert_eq!(entry.get(), &"first_value");
        assert_eq!(value, "second_value");
    }
}

