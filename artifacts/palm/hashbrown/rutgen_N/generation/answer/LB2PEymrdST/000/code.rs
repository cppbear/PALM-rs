// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 41);
    
    match map.entry("poneyland") {
        Entry::Occupied(mut entry) => {
            entry.and_modify(|e| { *e += 1 });
            assert_eq!(*entry.get(), 42);
        },
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_and_modify_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    match map.entry("not_existing") {
        Entry::Occupied(_) => panic!("Expected vacant entry"),
        Entry::Vacant(entry) => {
            entry.or_insert(42);
            assert_eq!(map["not_existing"], 42);
        },
    }
}

#[test]
fn test_and_modify_multiple_calls() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 41);

    for _ in 0..3 {
        match map.entry("poneyland") {
            Entry::Occupied(mut entry) => {
                entry.and_modify(|e| { *e += 1 });
            },
            Entry::Vacant(_) => panic!("Expected occupied entry"),
        }
    }
    assert_eq!(map["poneyland"], 44);
}

