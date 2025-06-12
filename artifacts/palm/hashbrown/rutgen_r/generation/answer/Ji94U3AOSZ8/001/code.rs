// Answer 0

#[test]
fn test_insert_entry_vacant() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    // Create a hash map with str keys and u32 values.
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Use EntryRef to get a VacantEntryRef to a key that doesn't exist yet.
    if let EntryRef::Vacant(v) = map.entry_ref("poneyland") {
        // Insert the entry
        let o = v.insert_entry(37);
        
        // Assertions to verify the OccupiedEntry is correct
        assert_eq!(o.get(), &37);
        
        // Check the internal HashMap state to ensure the key is associated with the expected value
        assert_eq!(map.get("poneyland"), Some(&37));
    }
}

#[test]
fn test_insert_entry_overwrite() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Initially insert a value for "poneyland"
    map.insert("poneyland", 10);
    
    // Now, we should get an OccupiedEntryRef instead of a VacantEntryRef
    if let EntryRef::Occupied(v) = map.entry_ref("poneyland") {
        let o = v.insert_entry(55);
        
        // Assertions to verify the updated value
        assert_eq!(o.get(), &55);
        assert_eq!(map.get("poneyland"), Some(&55));
    }
}

#[test]
#[should_panic]
fn test_insert_entry_on_vacant_key() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Attempting to insert into a VacantEntryRef for a key that doesn't exist
    if let EntryRef::Vacant(v) = map.entry_ref("nonexistent") {
        // This is the correct insertion that should not panic
        v.insert_entry(42);
    }
    
    // Now trying to access it again should not panic, simulating an invalid access
    if let EntryRef::Occupied(v) = map.entry_ref("nonexistent") {
        v.insert_entry(100); // This should panic, as "nonexistent" should not have occupied entry yet.
    }
}

