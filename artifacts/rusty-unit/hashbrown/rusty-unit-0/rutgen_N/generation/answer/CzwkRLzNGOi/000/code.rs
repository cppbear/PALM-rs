// Answer 0

#[test]
fn test_key_existing_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);
    let entry = map.entry("poneyland");

    assert_eq!(entry.key(), &"poneyland");
}

#[test]
fn test_key_nonexistent_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("horseland");

    assert_eq!(entry.key(), &"horseland");
}

