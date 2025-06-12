// Answer 0

#[test]
fn test_entry_key_occupied() {
    use hashbrown::HashMap;
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 1);
    let entry = map.entry("key1");
    let key = entry.key();
}

#[test]
fn test_entry_key_non_existent() {
    use hashbrown::HashMap;
    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("key3");
    let key = entry.key();
}

