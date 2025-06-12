// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use crate::hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    
    // Setup a HashMap with the default hasher
    let mut map: HashMap<&str, u32> = HashMap::new();
    
    // Insert a key-value pair into the map
    map.insert("poneyland", 3);
    
    // Create an Entry reference using the `entry` method
    let entry = map.entry("poneyland");
    
    // Modify the key using `or_insert_with`
    let value = entry.or_insert_with(|| 10);
    
    // The value should not change and should return mutable reference to the existing value
    assert_eq!(*value, 3);
    
    // Update the value through the returned mutable reference
    *value *= 2;
    
    // Validate that the value in the map is updated correctly
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use crate::hashbrown::HashMap;

    // Setup a new HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Create an Entry reference for a nonexistent key
    // This should lead to a Vacant entry
    let entry = map.entry("poneyland");

    // Insert a default value using `or_insert_with`
    let value = entry.or_insert_with(|| 3);
    
    // Ensure that the value has been inserted correctly
    assert_eq!(*value, 3);
    
    // Check the map to ensure the entry exists
    assert_eq!(map["poneyland"], 3);
}

