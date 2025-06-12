// Answer 0

#[test]
fn test_key_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);

    match map.entry("poneyland") {
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
        Entry::Occupied(entry) => {
            assert_eq!(entry.key(), &"poneyland");
        }
    }
}

#[test]
fn test_key_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let map: HashMap<&str, u32> = HashMap::new();

    match map.entry("nonexistent") {
        Entry::Vacant(_) => {} // Expecting a vacant entry, no panic
        Entry::Occupied(_) => panic!("Expected a vacant entry"),
    }
}

