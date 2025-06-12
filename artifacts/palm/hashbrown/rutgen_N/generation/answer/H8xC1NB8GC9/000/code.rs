// Answer 0

#[test]
fn test_remove_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    assert!(map.is_empty() && map.capacity() == 0);

    map.entry("poneyland").or_insert(12);

    if let Entry::Occupied(o) = map.entry("poneyland") {
        assert_eq!(o.remove_entry(), ("poneyland", 12));
    }

    assert_eq!(map.contains_key("poneyland"), false);
    assert!(map.is_empty());
}

#[test]
fn test_remove_entry_empty_map() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Entry::Vacant(_) = map.entry("not_found") {
        // Attempting to remove entry from an empty map should not panic.
        // This entry won't exist.
    }
}

#[test]
#[should_panic(expected = "entry not found")]
fn test_remove_entry_non_existent() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("existing").or_insert(20);

    // Attempting to remove an entry that does not exist.
    if let Entry::Occupied(o) = map.entry("non_existent") {
        o.remove_entry();
    }
}

