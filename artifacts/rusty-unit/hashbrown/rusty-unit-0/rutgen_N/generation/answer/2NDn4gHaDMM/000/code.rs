// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;
    use std::hash::Hash;
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("horseyland", 10);
    
    let mut entry = map.entry("horseyland");
    let occupied_entry = entry.insert(37);
    
    assert_eq!(occupied_entry.key(), &"horseyland");
    assert_eq!(map.get("horseyland"), Some(&37));
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let mut entry = map.entry("unicornland");
    let occupied_entry = entry.insert(42);
    
    assert_eq!(occupied_entry.key(), &"unicornland");
    assert_eq!(map.get("unicornland"), Some(&42));
}

#[test]
fn test_insert_with_existing_key_updates_value() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("mermaidland", 28);
    
    let mut entry = map.entry("mermaidland");
    let occupied_entry = entry.insert(100);
    
    assert_eq!(occupied_entry.key(), &"mermaidland");
    assert_eq!(map.get("mermaidland"), Some(&100));
}

