// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("horseyland".to_owned(), 0);
    
    let entry = map.entry_ref("horseyland").insert(37);
    
    assert_eq!(entry.key(), "horseyland");
    assert_eq!(entry.get(), &37);
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    let entry = map.entry_ref("vacantland").insert(42);
    
    assert_eq!(entry.key(), "vacantland");
    assert_eq!(entry.get(), &42);
}

#[test]
fn test_insert_overwrite() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("overwrite".to_owned(), 10);
    
    let entry = map.entry_ref("overwrite").insert(50);
    
    assert_eq!(entry.key(), "overwrite");
    assert_eq!(entry.get(), &50);
}

#[test]
fn test_insert_multiple_entries() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    let entry1 = map.entry_ref("first").insert(1);
    assert_eq!(entry1.key(), "first");
    assert_eq!(entry1.get(), &1);

    let entry2 = map.entry_ref("first").insert(2);
    assert_eq!(entry2.key(), "first");
    assert_eq!(entry2.get(), &2);
}

