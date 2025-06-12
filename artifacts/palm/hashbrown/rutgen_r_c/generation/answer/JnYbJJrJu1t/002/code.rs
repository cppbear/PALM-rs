// Answer 0

#[test]
fn test_and_replace_entry_with_exists() {
    use hashbrown::{HashMap, hash_map::{RawEntryMut, RawOccupiedEntryMut}};

    // Create a HashMap and insert an initial value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // Obtain a RawEntryMut from the map for the key "poneyland"
    let entry = map.raw_entry_mut().from_key("poneyland");

    // Call and_replace_entry_with to modify the value
    let updated_entry = entry.and_replace_entry_with(|k, v| {
        assert_eq!(k, &"poneyland");
        assert_eq!(v, 42);
        Some(v + 1)
    });

    match updated_entry {
        RawEntryMut::Occupied(e) => {
            assert_eq!(e.key(), &"poneyland");
            assert_eq!(e.get(), &43);
        },
        RawEntryMut::Vacant(_) => panic!(),
    }

    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_replace_entry_with_not_found() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    // Create a HashMap without initial values
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Obtain a RawEntryMut from the map for a non-existing key
    let entry = map.raw_entry_mut().from_key("poneyland");

    // Call and_replace_entry_with with a key that doesn't exist
    let updated_entry = entry.and_replace_entry_with(|_k, _v| {
        // This will never be called since the entry is vacant
        panic!();
    });

    match updated_entry {
        RawEntryMut::Vacant(_) => {}, // Expect Vacant status
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_remove_entry() {
    use hashbrown::{HashMap, hash_map::{RawEntryMut}};

    // Create a HashMap and insert an initial value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 42);

    // Obtain a RawEntryMut for the key "poneyland"
    let entry = map.raw_entry_mut().from_key("poneyland");

    // Call and_replace_entry_with to remove the entry
    let updated_entry = entry.and_replace_entry_with(|_k, _v| None);

    match updated_entry {
        RawEntryMut::Vacant(_) => {}, // Expect Vacant status
        RawEntryMut::Occupied(_) => panic!(),
    }

    assert!(!map.contains_key("poneyland"));
}

