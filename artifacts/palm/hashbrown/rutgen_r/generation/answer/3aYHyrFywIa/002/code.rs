// Answer 0

#[test]
fn test_or_default_with_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    
    // Testing with a vacant entry
    let entry = map.entry("unicornland");
    let default_value = entry.or_default();

    assert_eq!(default_value, &mut None);
    assert_eq!(map["unicornland"], None);
}

#[test]
fn test_or_default_with_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    
    // Inserting an existing key
    map.insert("dragonsland", Some(5));
    
    // Testing with an occupied entry
    let entry = map.entry("dragonsland");
    let occupied_value = entry.or_default();

    assert_eq!(occupied_value, &mut Some(5));
}

#[test]
fn test_or_default_multiple_vacant_entries() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    
    // Testing multiple vacant entries
    let entry1 = map.entry("mermaidland");
    let default_value1 = entry1.or_default();
    
    assert_eq!(default_value1, &mut None);
    assert_eq!(map["mermaidland"], None);
    
    let entry2 = map.entry("griffinland");
    let default_value2 = entry2.or_default();

    assert_eq!(default_value2, &mut None);
    assert_eq!(map["griffinland"], None);
}

#[test]
fn test_or_default_existing_value() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    
    // Inserting a key with a value
    map.insert("faerie", Some(10));
    
    // Testing existing value
    let entry = map.entry("faerie");
    let value = entry.or_default();

    assert_eq!(value, &mut Some(10));
}

