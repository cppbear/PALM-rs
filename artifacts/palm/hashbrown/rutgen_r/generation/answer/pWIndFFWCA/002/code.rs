// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;

    struct TestKey<'a>(&'a str);
    struct TestValue(u32);

    let mut map: HashMap<TestKey, TestValue> = HashMap::new();
    
    // Insert an initial value to occupy the entry
    map.insert(TestKey("occupied"), TestValue(5));

    // Use the or_insert method on an occupied entry
    let entry_ref = map.entry_ref(TestKey("occupied"));
    let value_ref = entry_ref.or_insert(TestValue(10));

    // Verify that the value was not changed, and we have a mutable reference
    assert_eq!(value_ref.0, 5);
    value_ref.0 *= 2; // Modify the value via mutable reference
    assert_eq!(map.get(&TestKey("occupied")).unwrap().0, 10);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;

    struct TestKey<'a>(&'a str);
    struct TestValue(u32);

    let mut map: HashMap<TestKey, TestValue> = HashMap::new();

    // Use the or_insert method on a vacant entry
    let entry_ref = map.entry_ref(TestKey("vacant"));
    let value_ref = entry_ref.or_insert(TestValue(15));

    // Verify that the value is inserted and we have a mutable reference
    assert_eq!(value_ref.0, 15);
    assert_eq!(map.get(&TestKey("vacant")).unwrap().0, 15);
}

