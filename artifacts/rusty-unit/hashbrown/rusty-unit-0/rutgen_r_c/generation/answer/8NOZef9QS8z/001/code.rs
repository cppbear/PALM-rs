// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use crate::HashMap;
    use crate::hash_map::Entry;

    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Insert initial value
    map.insert("poneyland", 12);
    
    // Check if the entry is occupied
    if let Entry::Occupied(mut entry) = map.entry("poneyland") {
        // Insert a new value and check the returned old value
        assert_eq!(entry.insert(15), 12);
        // Verify that the value in the map is updated
        assert_eq!(map["poneyland"], 15);
    } else {
        panic!("Expected an occupied entry.");
    }
}

#[test]
fn test_insert_new_entry() {
    use crate::HashMap;
    use crate::hash_map::Entry;

    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Insert into a new entry and expect the default value (if any)
    if let Entry::Vacant(entry) = map.entry("newland") {
        assert_eq!(entry.insert(10), 0); // Assuming default value
        assert_eq!(map["newland"], 10);
    } else {
        panic!("Expected a vacant entry.");
    }
} 

#[test]
fn test_insert_with_multiple_updates() {
    use crate::HashMap;
    use crate::hash_map::Entry;

    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Insert initial value
    map.insert("poneyland", 20);
    
    if let Entry::Occupied(mut entry) = map.entry("poneyland") {
        // Multiple inserts and checking old values
        assert_eq!(entry.insert(30), 20);
        assert_eq!(map["poneyland"], 30);
        assert_eq!(entry.insert(40), 30);
        assert_eq!(map["poneyland"], 40);
    } else {
        panic!("Expected an occupied entry.");
    }
} 

#[test]
#[should_panic(expected = "value not found")] // Customize according to specific panic conditions set by the Allocator
fn test_insert_panic_on_non_existing() {
    use crate::HashMap;
    use crate::hash_map::Entry;

    // Create a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Attempt to insert into a key that has never been inserted
    if let Entry::Occupied(_) = map.entry("poneyland") {
        panic!("This entry should not be occupied.");
    }
}

