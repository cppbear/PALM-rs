// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();

    // Nonexistent key
    map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count());
    assert_eq!(map["poneyland"], 9);
}

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();

    // Insert a value for the key
    map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count());
    assert_eq!(map["poneyland"], 9);

    // Existing key
    *map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count() * 10) *= 2;
    assert_eq!(map["poneyland"], 18);
}

