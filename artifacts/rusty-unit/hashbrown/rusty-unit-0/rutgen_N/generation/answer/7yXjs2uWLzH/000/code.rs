// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(o) = map.entry("poneyland") {
        o.insert(37);
    }
    assert_eq!(map["poneyland"], 37);
}

#[test]
fn test_insert_overwrite_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 37);
    
    if let Entry::Vacant(o) = map.entry("poneyland") {
        o.insert(42); // This won't actually run as the entry is not vacant
    }
    assert_eq!(map["poneyland"], 37); // Should still be 37
}

#[test]
fn test_insert_multiple_vacant_entries() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(o) = map.entry("poneyland") {
        o.insert(37);
    }
    if let Entry::Vacant(o) = map.entry("friendland") {
        o.insert(42);
    }
    
    assert_eq!(map["poneyland"], 37);
    assert_eq!(map["friendland"], 42);
}

#[test]
fn test_insert_empty_string_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(o) = map.entry("") {
        o.insert(99);
    }
    
    assert_eq!(map[""], 99);
}

