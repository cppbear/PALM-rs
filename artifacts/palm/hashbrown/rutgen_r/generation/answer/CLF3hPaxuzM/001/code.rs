// Answer 0

#[test]
fn test_search_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::raw_entry::RawEntryMut;

    // Create a simple hash map to use with RawEntryMut
    let mut map: HashMap<u64, &str> = HashMap::new();
    map.insert(42, "answer");
    map.insert(13, "unlucky");

    // Create a structure to hold the map and provide the search method
    struct MyMap<K, V> {
        map: HashMap<K, V>,
    }

    let my_map = MyMap { map };

    let hash = 42;

    // Perform the search with a matching condition
    let entry = my_map.map.raw_entry_mut().search(hash, |k| *k == hash);

    // Check that the entry is occupied
    if let RawEntryMut::Occupied(_) = entry {
        // The search returned an occupied entry, so this test succeeds
        assert!(true);
    } else {
        // This should not happen if our test is set up correctly
        panic!("Expected an occupied entry, but found a vacant entry.");
    }
}

#[test]
fn test_search_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::raw_entry::RawEntryMut;

    // Create a simple hash map to use with RawEntryMut
    let mut map: HashMap<u64, &str> = HashMap::new();
    map.insert(42, "answer");
    map.insert(13, "unlucky");

    // Create a structure to hold the map and provide the search method
    struct MyMap<K, V> {
        map: HashMap<K, V>,
    }

    let my_map = MyMap { map };

    let hash = 99; // A hash that does not exist in the map

    // Perform the search with a condition that does not match
    let entry = my_map.map.raw_entry_mut().search(hash, |k| *k == hash);

    // Check that the entry is vacant
    if let RawEntryMut::Vacant(_) = entry {
        // The search returned a vacant entry, so this test succeeds
        assert!(true);
    } else {
        // This should not happen if our test is set up correctly
        panic!("Expected a vacant entry, but found an occupied entry.");
    }
}

