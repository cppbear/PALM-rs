// Answer 0

#[test]
fn test_into_key_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.entry("poneyland") {
        Entry::Occupied(_) => panic!(),
        Entry::Vacant(v) => {
            assert_eq!(v.into_key(), "poneyland");
        },
    }
}

#[test]
fn test_into_key_vacant_entry_boundary() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Test with an empty map for a key that has not been inserted yet
    match map.entry("") {
        Entry::Occupied(_) => panic!(),
        Entry::Vacant(v) => {
            assert_eq!(v.into_key(), "");
        },
    }
}

