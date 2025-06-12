// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);
    
    if let Entry::Occupied(o) = map.entry("poneyland") {
        assert_eq!(o.remove(), 12);
    }

    assert_eq!(map.contains_key("poneyland"), false);
    assert!(map.is_empty());
}

#[test]
fn test_remove_non_existent_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    assert!(map.is_empty());
    
    // Attempt to remove a non-existent entry
    if let Entry::Vacant(v) = map.entry("nonexistent") {
        assert!(std::panic::catch_unwind(|| { v.remove(); }).is_err());
    }
}

