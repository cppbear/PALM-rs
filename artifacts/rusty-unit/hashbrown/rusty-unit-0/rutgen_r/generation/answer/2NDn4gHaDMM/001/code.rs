// Answer 0

#[test]
fn test_insert_on_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    // Initialize a HashMap which will be empty (this ensures the entry is vacant)
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Attempting to insert into an empty HashMap (Entry::Vacant condition)
    let entry = map.entry("vacantland").or_insert(0); // Using or_insert to ensure we start with a vacant entry
    *entry = 42; // Insert new value directly

    // Assert that the map contains the new entry
    assert_eq!(map.get("vacantland"), Some(&42));

    // Verify that we get the expected entry key
    if let Entry::Occupied(occupied_entry) = map.entry("vacantland") {
        assert_eq!(occupied_entry.key(), &"vacantland");
        assert_eq!(*occupied_entry.get(), 42);
    } else {
        panic!("Expected an occupied entry.");
    }
} 

#[test]
fn test_insert_on_existing_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    // Initialize a HashMap and insert a value to begin with
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existingland", 25);

    // Now we'll insert a new value using the same key, simulating `Entry::Occupied`
    let entry = map.entry("existingland").or_insert(0);
    let old_value = *entry; 
    *entry = 50; // Insert new value

    // Assert that the map reflects the updated value
    assert_eq!(map.get("existingland"), Some(&50));

    // Verify the old value was correctly replaced
    assert_eq!(old_value, 25); // Initial value before insertion
    assert_eq!(*entry, 50); // Updated value after insertion
} 

#[should_panic]
#[test]
fn test_insert_on_vacant_entry_panic() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    // Initialize an empty HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();

    // This should trigger a panic since we're trying to `insert` on a nonexistent Entry
    if let Entry::Vacant(entry) = map.entry("nonexistentland") {
        entry.insert(100); // Should insert successfully
    }

    // Now we'll attempt a second insert on the same key, which should panic
    if let Entry::Vacant(_) = map.entry("nonexistentland") {
        panic!("This entry is still vacant and should not be allowed to insert a second time!");
    }
}

