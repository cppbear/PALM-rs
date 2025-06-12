// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);

    // Existing key, should modify the value
    let value = map.entry("poneyland").or_insert_with(|| 10);
    *value *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Nonexistent key, should insert the value from the closure
    let value = map.entry("unicornland").or_insert_with(|| 5);
    assert_eq!(*value, 5);
}

