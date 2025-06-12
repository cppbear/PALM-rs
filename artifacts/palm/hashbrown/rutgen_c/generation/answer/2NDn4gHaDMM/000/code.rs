// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("horseyland", 20);

    let entry = map.entry("horseyland");
    let occupied_entry = entry.insert(37);

    assert_eq!(occupied_entry.key(), &"horseyland");
    assert_eq!(occupied_entry.get(), &37);
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("unicornland");
    let occupied_entry = entry.insert(42);

    assert_eq!(occupied_entry.key(), &"unicornland");
    assert_eq!(occupied_entry.get(), &42);
}

#[test]
fn test_insert_overwrite_entry() {
    use hashbrown::HashMap;
    
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("mermaid", 15);

    let entry = map.entry("mermaid");
    let occupied_entry = entry.insert(50);

    assert_eq!(occupied_entry.key(), &"mermaid");
    assert_eq!(occupied_entry.get(), &50);
}

