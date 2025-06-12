// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;

    struct MyKey(String);
    struct MyValue(u32);

    let mut map: HashMap<MyKey, MyValue> = HashMap::new();

    // Test inserting with vacant entry
    let entry_ref = map.entry_ref(MyKey("test".to_string()));
    let value_ref = entry_ref.or_insert_with(|| MyValue(42));
    assert_eq!(value_ref.0, 42); // Check the inserted value
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;

    struct MyKey(String);
    struct MyValue(u32);

    let mut map: HashMap<MyKey, MyValue> = HashMap::new();
    map.insert(MyKey("existing".to_string()), MyValue(10));

    // Test inserting with occupied entry
    let entry_ref = map.entry_ref(MyKey("existing".to_string()));
    {
        let value_ref = entry_ref.or_insert_with(|| MyValue(20));
        assert_eq!(value_ref.0, 10); // Check the existing value
    }
    
    // Check that the value has not changed
    assert_eq!(map[&MyKey("existing".to_string())].0, 10);
}

#[test]
fn test_or_insert_with_multiple_calls() {
    use hashbrown::HashMap;

    struct MyKey(String);
    struct MyValue(u32);

    let mut map: HashMap<MyKey, MyValue> = HashMap::new();

    // Insert first value
    let entry_ref = map.entry_ref(MyKey("double".to_string()));
    let value_ref = entry_ref.or_insert_with(|| MyValue(3));
    assert_eq!(value_ref.0, 3); // First insertion

    // Modify the entry and insert again
    {
        *value_ref = MyValue(6);
        let new_ref = entry_ref.or_insert_with(|| MyValue(10));
        assert_eq!(new_ref.0, 6); // Check that the previous value was kept
    }

    // Check final value
    assert_eq!(map[&MyKey("double".to_string())].0, 6);
}

