// Answer 0

#[test]
fn test_or_insert_with_nonexistent_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Test inserting with a non-existent key
    map.entry("poneyland").or_insert_with(|| 3);
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_with_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Insert initial value
    map.entry("poneyland").or_insert_with(|| 3);
    assert_eq!(map["poneyland"], 3);

    // Test modifying existing key
    *map.entry("poneyland").or_insert_with(|| 10) *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // Ensure a value is inserted in an empty map
    let value = map.entry("empty").or_insert_with(|| 42);
    assert_eq!(*value, 42);
    assert_eq!(map["empty"], 42);
}

#[test]
fn test_or_insert_with_multiple_insertions() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();

    // First insertion
    map.entry("duplicate").or_insert_with(|| 10);
    assert_eq!(map["duplicate"], 10);

    // Second insertion
    *map.entry("duplicate").or_insert_with(|| 20) += 5;
    assert_eq!(map["duplicate"], 15);
}

