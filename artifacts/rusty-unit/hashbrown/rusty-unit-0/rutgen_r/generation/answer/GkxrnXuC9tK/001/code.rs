// Answer 0

#[test]
fn test_insert_unique_unchecked_valid() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    
    unsafe {
        let (key1, value1) = map.insert_unique_unchecked(1, "a");
        assert_eq!(key1, &1);
        assert_eq!(value1, &mut "a");

        let (key2, value2) = map.insert_unique_unchecked(2, "b");
        assert_eq!(key2, &2);
        assert_eq!(value2, &mut "b");

        let (key3, value3) = map.insert_unique_unchecked(3, "c");
        assert_eq!(key3, &3);
        assert_eq!(value3, &mut "c");

        assert_eq!(map.len(), 3);
    }
}

#[test]
#[should_panic]
fn test_insert_unique_unchecked_existing_key() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    
    unsafe {
        let _ = map.insert_unique_unchecked(1, "a");
        let _ = map.insert_unique_unchecked(1, "b"); // This should panic
    }
}

#[test]
fn test_insert_unique_unchecked_update_value() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    
    unsafe {
        let (_, value) = map.insert_unique_unchecked(1, "a");
        *value = "updated_value";
        
        assert_eq!(map[&1], "updated_value");
    }
}

#[test]
fn test_insert_unique_unchecked_multiple_inserts() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    
    unsafe {
        let (key1, value1) = map.insert_unique_unchecked(1, "first");
        let (key2, value2) = map.insert_unique_unchecked(2, "second");
        
        assert_eq!(key1, &1);
        assert_eq!(value1, &mut "first");
        assert_eq!(key2, &2);
        assert_eq!(value2, &mut "second");
        
        assert_eq!(map.len(), 2);
    }
}

