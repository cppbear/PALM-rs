// Answer 0

#[test]
fn test_entry_ref_key_occupied() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_owned(), 3);
    
    let entry_ref = map.entry_ref("poneyland");
    if let EntryRef::Occupied(entry) = entry_ref {
        assert_eq!(entry.key(), "poneyland");
    } else {
        panic!("Expected an occupied entry");
    }
}

#[test]
fn test_entry_ref_key_vacant() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<String, u32> = HashMap::new();
    
    let entry_ref = map.entry_ref("horseland");
    if let EntryRef::Vacant(entry) = entry_ref {
        assert_eq!(entry.key(), "horseland");
    } else {
        panic!("Expected a vacant entry");
    }
}

