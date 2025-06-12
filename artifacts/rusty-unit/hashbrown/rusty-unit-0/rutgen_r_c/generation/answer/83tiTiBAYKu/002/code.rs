// Answer 0

#[test]
fn test_entry_ref_occupied() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    // Create a struct to act as a key for our HashMap
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct CustomKey(String);

    // Instantiate the HashMap with CustomKey as key type
    let mut map: HashMap<CustomKey, usize> = HashMap::new();
    
    // Insert a key-value pair into the HashMap
    let key = CustomKey("example".to_owned());
    map.insert(key.clone(), 42);
    
    // Create an entry_ref using the same key
    let entry = map.entry_ref(&key);
    
    // Assert that we get an Occupied entry reference
    match entry {
        EntryRef::Occupied(occupied_entry) => {
            assert_eq!(occupied_entry.elem.ptr.as_ref(), &(key, 42).into());
        },
        EntryRef::Vacant(_) => panic!("Expected an occupied entry reference"),
    }
}

#[test]
fn test_entry_ref_vacant() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    // Create a struct to act as a key for our HashMap
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct CustomKey(String);

    // Instantiate the HashMap with CustomKey as key type
    let mut map: HashMap<CustomKey, usize> = HashMap::new();
    
    // Create an entry_ref using a key that is not in the HashMap
    let key = CustomKey("nonexistent".to_owned());
    let entry = map.entry_ref(&key);
    
    // Assert that we get a Vacant entry reference
    match entry {
        EntryRef::Vacant(vacant_entry) => {
            assert_eq!(vacant_entry.key, &key);
        },
        EntryRef::Occupied(_) => panic!("Expected a vacant entry reference"),
    }
}

