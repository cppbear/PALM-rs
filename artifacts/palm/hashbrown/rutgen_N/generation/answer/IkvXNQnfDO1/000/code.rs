// Answer 0

#[test]
fn test_remove_function_when_entry_exists() {
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
fn test_remove_function_when_entry_does_not_exist() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    assert!(map.is_empty());

    if let Entry::Vacant(v) = map.entry("poneyland") {
        // This entry should not exist to test the next logic
        v.insert(12);
    }

    // Now remove it to see it works
    if let Entry::Occupied(o) = map.entry("poneyland") {
        assert_eq!(o.remove(), 12);
    }

    assert_eq!(map.contains_key("poneyland"), false);
    assert!(map.is_empty());
}

