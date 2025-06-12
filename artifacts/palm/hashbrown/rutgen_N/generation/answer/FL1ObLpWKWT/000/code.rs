// Answer 0

#[test]
fn test_insert_vacant_entry_ref() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<String, u32> = HashMap::new();
    let key: &str = "poneyland";

    if let EntryRef::Vacant(o) = map.entry_ref(key) {
        o.insert(37);
    }
    
    assert_eq!(map["poneyland"], 37);
}

#[test]
fn test_insert_multiple_vacant_entry_refs() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<String, u32> = HashMap::new();
    let keys = vec!["key1", "key2", "key3"];

    for key in keys {
        if let EntryRef::Vacant(o) = map.entry_ref(key) {
            o.insert(10);
        }
    }

    assert_eq!(map["key1"], 10);
    assert_eq!(map["key2"], 10);
    assert_eq!(map["key3"], 10);
}

#[test]
fn test_insert_boundary_condition() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<String, u32> = HashMap::new();
    let key: &str = "boundary_key";

    if let EntryRef::Vacant(o) = map.entry_ref(key) {
        o.insert(u32::MAX); // Test boundary value
    }

    assert_eq!(map["boundary_key"], u32::MAX);
}

