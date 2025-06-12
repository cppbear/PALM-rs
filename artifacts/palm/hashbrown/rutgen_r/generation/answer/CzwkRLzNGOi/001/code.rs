// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);
    
    let entry = map.entry("poneyland");
    match entry {
        Entry::Occupied(ref entry) => {
            assert_eq!(entry.key(), &"poneyland");
        },
        _ => panic!("Expected Occupied entry"),
    }
}

#[test]
fn test_key_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let entry = map.entry("horseland");
    match entry {
        Entry::Vacant(ref entry) => {
            assert_eq!(entry.key(), &"horseland");
        },
        _ => panic!("Expected Vacant entry"),
    }
}

