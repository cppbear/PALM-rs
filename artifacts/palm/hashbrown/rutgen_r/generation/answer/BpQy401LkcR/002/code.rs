// Answer 0

#[test]
fn test_or_insert_with_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 3);

    // Utilize the method on an existing key
    let value: &mut u32 = map.entry_ref("poneyland").or_insert_with(|| 10);
    *value *= 2; // This should modify the existing value

    assert_eq!(map["poneyland"], 6); // Expecting the modified value
}

#[test]
fn test_or_insert_with_nonexistent_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();

    // Utilize the method on a nonexistent key
    map.entry_ref("poneyland").or_insert_with(|| 3);
    assert_eq!(map["poneyland"], 3); // Expecting the inserted value
}

#[test]
fn test_or_insert_with_multiple_insertions() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 10);

    // First call does not modify the existing entry
    let value1: &mut u32 = map.entry_ref("poneyland").or_insert_with(|| 5);
    *value1 += 5; // value should now be 15

    assert_eq!(map["poneyland"], 15); // Expecting modified value

    // Second call with a different value
    let value2: &mut u32 = map.entry_ref("poneyland").or_insert_with(|| 100);
    *value2 += 10; // value should now be 25

    assert_eq!(map["poneyland"], 25); // Ensuring the value is updated correctly
}

#[test]
#[should_panic]
fn test_or_insert_with_panic_on_drop() {
    use hashbrown::HashMap;

    struct PanicOnDrop;

    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("Panic on drop");
        }
    }

    let mut map: HashMap<String, PanicOnDrop> = HashMap::new();
    map.insert("poneyland".to_string(), PanicOnDrop);

    // This should panic due to the drop implementation when trying to insert a value
    let _value: &mut PanicOnDrop = map.entry_ref("poneyland").or_insert_with(|| PanicOnDrop);
}

