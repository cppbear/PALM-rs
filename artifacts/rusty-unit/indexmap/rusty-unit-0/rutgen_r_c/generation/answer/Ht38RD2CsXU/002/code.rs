// Answer 0

#[test]
fn test_from_hash_with_occupied_entry() {
    use crate::{IndexMap, HashValue};
    // Define a simple string key and i32 value for our test
    let mut map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert("key".into(), 42);

    // Assuming that the underlying structure has been set up properly
    let hash = 12345; // using a hash that we assume matches "key"
    
    // Create a RawEntryBuilderMut
    let mut builder = RawEntryBuilderMut { map: &mut map };

    // Create a matching function for the key
    let is_match = |key: &String| key == "key";

    // Call from_hash which should find the existing entry
    let entry = builder.from_hash(hash, is_match);
    
    // Validate that we received an Occupied entry
    if let RawEntryMut::Occupied(_occupied) = entry {
        // Unlocking the entry to verify its contents
        assert_eq!(_occupied.entries.get(_occupied.index), Some(&Bucket { hash: HashValue::new(hash), key: "key".into(), value: 42 }));
    } else {
        panic!("Expected RawEntryMut::Occupied but got Vacant");
    }
}

#[test]
fn test_from_hash_with_vacant_entry() {
    use crate::{IndexMap, HashValue};
    // Define a simple string key and i32 value for our test
    let mut map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    
    // Assuming the map is empty
    let hash = 54321; // using a hash that does not match any entry
    
    // Create a RawEntryBuilderMut
    let mut builder = RawEntryBuilderMut { map: &mut map };

    // Create a matching function that will not find a key
    let is_match = |key: &String| key == "key";

    // Call from_hash which should not find the existing entry
    let entry = builder.from_hash(hash, is_match);
    
    // Validate that we received a Vacant entry
    if let RawEntryMut::Vacant(_vacant) = entry {
        // We can check that the map is still empty
        assert!(map.is_empty());
    } else {
        panic!("Expected RawEntryMut::Vacant but got Occupied");
    }
}

