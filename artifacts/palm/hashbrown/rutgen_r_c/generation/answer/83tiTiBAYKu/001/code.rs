// Answer 0

#[test]
fn test_entry_ref_vacant_case() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHashBuilder;

    // Step 1: Create a new HashMap
    let mut map: HashMap<String, usize, DefaultHashBuilder> = HashMap::new();

    // Step 2: Create a reference to a non-existing key
    let key = "non_existent_key";

    // Step 3: Call entry_ref to check vacant entry
    let entry = map.entry_ref(&key);

    // Step 4: Verify that we get a Vacant entry
    match entry {
        hashbrown::hash_map::EntryRef::Vacant(vacant_entry) => {
            assert_eq!(vacant_entry.key, &key);
            assert!(vacant_entry.table.allocation_size() == 0);
        },
        _ => panic!("Expected a Vacant entry, but got Occupied"),
    }
}

#[test]
fn test_entry_ref_occupied_case() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHashBuilder;

    // Step 1: Create a new HashMap and insert a key-value pair
    let mut map: HashMap<String, usize, DefaultHashBuilder> = HashMap::new();
    map.insert("existing_key".to_string(), 42);

    // Step 2: Call entry_ref with the existing key
    let entry = map.entry_ref(&"existing_key".to_string());

    // Step 3: Verify we get an Occupied entry
    match entry {
        hashbrown::hash_map::EntryRef::Occupied(occupied_entry) => {
            assert_eq!(occupied_entry.table.get(&"existing_key").unwrap(), &42);
        },
        _ => panic!("Expected an Occupied entry, but got Vacant"),
    }
}

