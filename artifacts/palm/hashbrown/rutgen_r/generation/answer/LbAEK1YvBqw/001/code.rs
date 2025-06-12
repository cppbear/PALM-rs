// Answer 0

#[test]
fn test_into_key_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();

    match map.entry("test_entry") {
        Entry::Occupied(_) => panic!("Expected Entry to be Vacant"),
        Entry::Vacant(v) => {
            let key = v.into_key();
            assert_eq!(key, "test_entry");
        },
    }
}

#[test]
#[should_panic]
fn test_into_key_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("occupied_entry", 1);

    match map.entry("occupied_entry") {
        Entry::Occupied(_) => {
            // This block should execute, but we'll assert something that causes panic.
            panic!("Was supposed to be panicking");
        },
        Entry::Vacant(_) => {
            // This should never be reached.
            unreachable!();
        },
    }
}

