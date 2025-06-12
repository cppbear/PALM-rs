// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: HashMap<String, u32> = HashMap::new();
    let entry = map.entry_ref("horseyland").insert(37);

    assert_eq!(entry.key(), "horseyland");
    assert_eq!(*entry.get(), 37);
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: HashMap<String, u32> = HashMap::new();
    let entry = map.entry_ref("vacantland").insert(42);

    assert_eq!(entry.key(), "vacantland");
    assert_eq!(*entry.get(), 42);
}

#[test]
#[should_panic(expected = "key not found")]
fn test_insert_on_non_existent_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    let entry = map.entry_ref("non_existent").insert(10); // Should panic because the key doesn't exist
}

