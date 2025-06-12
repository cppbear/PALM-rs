// Answer 0

#[test]
fn test_hasher() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    // Initialize with a DefaultHashBuilder
    let hasher = DefaultHashBuilder::default();
    let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);

    // Obtain the hasher from the map
    let obtained_hasher: &DefaultHashBuilder = map.hasher();

    // Verifying that the obtained hasher is the same as the one used to create the map
    assert!(std::ptr::eq(obtained_hasher, &map.hash_builder));
}

#[test]
fn test_hasher_empty_map() {
    use hashbrown::HashMap;
    use hashbrown::DefaultHashBuilder;

    // Create an empty HashMap with a DefaultHashBuilder
    let hasher = DefaultHashBuilder::default();
    let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);

    // Obtain the hasher from the empty map
    let obtained_hasher: &DefaultHashBuilder = map.hasher();

    // Verify the hasher is still the same
    assert!(std::ptr::eq(obtained_hasher, &map.hash_builder));
}

