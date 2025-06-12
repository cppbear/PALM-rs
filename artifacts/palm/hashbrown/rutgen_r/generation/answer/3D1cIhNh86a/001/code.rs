// Answer 0

#[test]
fn test_get_mut_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        // Get mutable reference and modify the value
        *o.get_mut() += 10;
        assert_eq!(*o.get(), 22);
    }

    assert_eq!(map["poneyland"], 22);
}

#[test]
fn test_get_mut_multiple_modifications() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);

    if let Entry::Occupied(mut o) = map.entry("poneyland") {
        *o.get_mut() += 10;
        assert_eq!(*o.get(), 22);
        *o.get_mut() += 5;
    }

    assert_eq!(map["poneyland"], 27);
}

#[test]
fn test_get_mut_handle_non_existing_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);

    // This will not panic as we are only testing the modification of existing entries.
    let entry = map.entry("not_existing");
    assert_eq!(entry.or_insert(5), &mut 5);

    // Since "not_existing" was just added, we verify its value
    assert_eq!(map["not_existing"], 5);
}

#[test]
#[should_panic]
fn test_get_mut_panic_on_invalid_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Try to get mutable reference from an entry that does not exist
    let entry = map.entry("nonexistent");
    let _ = entry.get_mut(); // This should trigger a panic
}

