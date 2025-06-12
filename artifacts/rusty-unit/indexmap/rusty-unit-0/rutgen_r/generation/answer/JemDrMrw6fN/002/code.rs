// Answer 0

#[test]
fn test_or_insert_with_occupied() {
    use indexmap::IndexMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    // Create a type for our key and value
    type Key = i32;
    type Value = String;

    // Define a hasher type
    type Hasher = BuildHasherDefault<DefaultHasher>;

    // Create a new IndexMap and insert an initial value
    let mut map: IndexMap<Key, Value, Hasher> = IndexMap::with_capacity(1);
    map.insert(1, "initial_value".to_string());

    // Create a mutable reference to the entry
    let entry = map.entry(1);

    // Test the or_insert_with function when the entry is occupied
    let (key, value) = entry.or_insert_with(|| (2, "new_value".to_string()));

    // Validate the output
    assert_eq!(*key, 1);
    assert_eq!(*value, "initial_value");

    // Validate that the value in the map remains unchanged
    assert_eq!(map.get(&1).unwrap(), "initial_value");
}

