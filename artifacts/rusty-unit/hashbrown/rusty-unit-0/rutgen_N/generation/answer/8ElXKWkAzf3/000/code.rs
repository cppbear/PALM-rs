// Answer 0

#[test]
fn test_get_key_value() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    // Initialize a HashMap
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    // Access the raw entry mutable interface
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!("Expected entry to be occupied"),
        RawEntryMut::Occupied(o) => {
            // Test the get_key_value method
            assert_eq!(o.get_key_value(), (&"a", &100));
        }
    }
}

#[test]
fn test_get_key_value_non_existent_key() {
    use hashbrown::hash_map::{HashMap, RawEntryMut};

    // Initialize a HashMap
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

    // Try to access a non-existent key
    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {}, // Expecting it to be vacant
        RawEntryMut::Occupied(_) => panic!("Expected entry to be vacant"),
    }
}

