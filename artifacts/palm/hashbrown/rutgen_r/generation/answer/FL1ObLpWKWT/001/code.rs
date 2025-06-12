// Answer 0

#[test]
fn test_insert_vacant_entry_ref() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<String, u32> = HashMap::new();
    let key: &str = "poneyland";

    // Test case: inserting a value into a vacant entry
    if let EntryRef::Vacant(o) = map.entry_ref(key) {
        let value_ref: &mut u32 = o.insert(37);
        assert_eq!(*value_ref, 37);
    }

    assert_eq!(map["poneyland"], 37);

    // Test case: verifying that inserting a different value works without panic
    if let EntryRef::Vacant(o) = map.entry_ref("rainbowland") {
        let value_ref: &mut u32 = o.insert(42);
        assert_eq!(*value_ref, 42);
    }

    assert_eq!(map["rainbowland"], 42);
}

#[test]
#[should_panic]
fn test_insert_panic_on_existing_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<String, u32> = HashMap::new();
    let key: &str = "poneyland";
    
    // First insert a value
    if let EntryRef::Vacant(o) = map.entry_ref(key) {
        o.insert(37); // Insert initial value
    }

    // This should panic as we're trying to insert into an existing key
    if let EntryRef::Vacant(o) = map.entry_ref(key) {
        o.insert(99); // Attempting to insert again
    }
}

