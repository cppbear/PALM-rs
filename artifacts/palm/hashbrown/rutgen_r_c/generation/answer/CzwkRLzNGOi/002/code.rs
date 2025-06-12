// Answer 0

#[test]
fn test_entry_key_occupied() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);

    let entry = map.entry("poneyland");
    match entry {
        Entry::Occupied(ref occupied_entry) => {
            assert_eq!(occupied_entry.key(), &"poneyland");
        },
        Entry::Vacant(_) => panic!("Expected occupied entry, found vacant"),
    }
}

#[test]
fn test_entry_key_vacant() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map.entry("horseland");
    match entry {
        Entry::Occupied(_) => panic!("Expected vacant entry, found occupied"),
        Entry::Vacant(ref vacant_entry) => {
            assert_eq!(vacant_entry.key(), &"horseland");
        },
    }
}

