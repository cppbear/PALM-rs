// Answer 0

#[test]
fn test_insert_unique_unchecked() {
    use hashbrown::HashMap;

    // Create a new HashMap
    let mut map = HashMap::new();

    // Ensure initial size is 0
    assert_eq!(map.len(), 0);

    // Insert unique key-value pairs using the unsafe method
    let (key1, value1) = unsafe { map.insert_unique_unchecked(1, "a") };
    assert_eq!(key1, &1);
    assert_eq!(*value1, "a");

    let (key2, value2) = unsafe { map.insert_unique_unchecked(2, "b") };
    assert_eq!(key2, &2);
    assert_eq!(*value2, "b");

    let (key3, value3) = unsafe { map.insert_unique_unchecked(3, "c") };
    assert_eq!(key3, &3);
    assert_eq!(*value3, "c");

    // Check if size is 3
    assert_eq!(map.len(), 3);

    // Modify one of the values
    *value2 = "modified_b";

    // Validate the values in the map
    assert_eq!(map[&1], "a");
    assert_eq!(map[&2], "modified_b");
    assert_eq!(map[&3], "c");

    // Insert another unique item
    let (key4, value4) = unsafe { map.insert_unique_unchecked(4, "d") };
    assert_eq!(key4, &4);
    assert_eq!(*value4, "d");

    // Check size again
    assert_eq!(map.len(), 4);
    
    // Modify the newly inserted value
    *value4 = "modified_d";

    // Final state checks
    assert_eq!(map[&1], "a");
    assert_eq!(map[&2], "modified_b");
    assert_eq!(map[&3], "c");
    assert_eq!(map[&4], "modified_d");
}

