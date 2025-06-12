// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert(String::from("poneyland"), 3);
    
    let entry = map.entry_ref("poneyland");
    assert_eq!(entry.key(), "poneyland");
}

#[test]
fn test_key_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    let entry = map.entry_ref("horseland");
    assert_eq!(entry.key(), "horseland");
}

