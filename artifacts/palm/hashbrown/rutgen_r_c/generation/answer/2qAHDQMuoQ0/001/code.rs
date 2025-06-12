// Answer 0

#[test]
fn test_into_mut_with_valid_entry() {
    use hashbrown::hash_map::{Entry, HashMap};
    
    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();
    // Insert an initial value
    map.insert("poneyland", 12);

    // Obtain a mutable reference to the entry
    if let Entry::Occupied(mut entry) = map.entry("poneyland") {
        let value: &mut u32 = entry.into_mut();
        *value += 10; // Modify the value
        assert_eq!(map["poneyland"], 22); // Verify the modification
    } else {
        panic!("Expected an occupied entry");
    }
}

#[test]
#[should_panic]
fn test_into_mut_with_vacant_entry() {
    use hashbrown::hash_map::{Entry, HashMap};
    
    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Try to obtain a mutable reference to a non-existing entry "vacantland"
    match map.entry("vacantland") {
        Entry::Occupied(_) => panic!("Expected a vacant entry"),
        Entry::Vacant(_) => {} // This is expected
    }
}

