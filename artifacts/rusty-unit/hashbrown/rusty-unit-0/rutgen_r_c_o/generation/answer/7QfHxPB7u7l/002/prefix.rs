// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 3);
    
    let entry_ref = map.entry_ref("poneyland");
    let key = entry_ref.key();
}

#[test]
fn test_key_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    let entry_ref = map.entry_ref("horseland");
    let key = entry_ref.key();
}

#[test]
fn test_key_multiple_entries_occupied() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    for c in 'a' as u8..='z' as u8 {
        map.insert((c as char).to_string(), c as u32);
    }

    let entry_ref = map.entry_ref("a");
    let key_a = entry_ref.key();

    let entry_ref = map.entry_ref("z");
    let key_z = entry_ref.key();
}

#[test]
fn test_key_multiple_entries_vacant() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    for c in 'a' as u8..='j' as u8 {
        map.insert((c as char).to_string(), c as u32);
    }

    let entry_ref = map.entry_ref("k");
    let key_k = entry_ref.key();
}

