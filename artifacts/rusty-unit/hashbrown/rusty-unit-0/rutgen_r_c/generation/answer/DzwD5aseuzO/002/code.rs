// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashMap, raw::{RawEntryMut, RawOccupiedEntryMut, RawVacantEntryMut}};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    struct CustomAllocator; // Implement as needed

    let mut map: HashMap<&str, String, BuildHasherDefault<RandomState>> = HashMap::new();
    map.insert("key1", "value1".to_string());

    let entry = {
        let mut raw_entry = map.raw_entry_mut().from_key("key1");
        match raw_entry {
            RawEntryMut::Occupied(entry) => entry,
            RawEntryMut::Vacant(_) => panic!("Expected occupied entry"),
        }
    };

    let (key_ref, value_ref) = entry.or_insert_with(|| {
        ("new_key".to_string(), "new_value".to_string())
    });

    assert_eq!(*key_ref, "key1");
    assert_eq!(*value_ref, "value1".to_string());
}

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::{HashMap, raw::{RawEntryMut, RawOccupiedEntryMut, RawVacantEntryMut}};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    struct CustomAllocator; // Implement as needed

    let mut map: HashMap<&str, String, BuildHasherDefault<RandomState>> = HashMap::new();

    let entry = {
        let mut raw_entry = map.raw_entry_mut().from_key("key2");
        match raw_entry {
            RawEntryMut::Occupied(_) => panic!("Expected vacant entry"),
            RawEntryMut::Vacant(entry) => entry,
        }
    };

    let (key_ref, value_ref) = entry.or_insert_with(|| {
        ("key2".to_string(), "value2".to_string())
    });

    assert_eq!(*key_ref, "key2");
    assert_eq!(*value_ref, "value2".to_string());
}

