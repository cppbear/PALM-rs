// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::hash_map::{RawEntryMut, RawOccupiedEntryMut, HashMap};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);
    
    let entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        elem: map.get_key_value_mut("key1").unwrap(),
        table: &mut map,
        hash_builder: &BuildHasherDefault::<RandomState>::default(),
    }).insert("key1", 20);

    assert_eq!(entry.remove_entry(), ("key1", 20));
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::hash_map::{RawEntryMut, RawVacantEntryMut, HashMap};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let entry = RawEntryMut::Vacant(RawVacantEntryMut {
        table: &mut map,
        hash_builder: &BuildHasherDefault::<RandomState>::default(),
    }).insert("new_key", 30);

    assert_eq!(map.get("new_key"), Some(&30));
}

#[test]
fn test_insert_empty_map() {
    use hashbrown::hash_map::{RawEntryMut, RawVacantEntryMut, HashMap};
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let entry = RawEntryMut::Vacant(RawVacantEntryMut {
        table: &mut map,
        hash_builder: &BuildHasherDefault::<RandomState>::default(),
    }).insert("key", 45);

    assert_eq!(entry, (&"key", &45));
}

