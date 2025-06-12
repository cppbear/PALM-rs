// Answer 0

#[test]
fn test_and_replace_entry_with_vacant() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .raw_entry_mut()
        .from_key("nonexistent_key")
        .and_replace_entry_with(|_k, _v| Some(1));

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_empty_map() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .raw_entry_mut()
        .from_key("missing_key")
        .and_replace_entry_with(|_k, _v| Some(100));

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_after_clear() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);
    map.clear();

    let entry = map
        .raw_entry_mut()
        .from_key("key1")
        .and_replace_entry_with(|_k, _v| None);

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_after_remove() {
    use hashbrown::{HashMap, hash_map::RawEntryMut};

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("existing_key", 20);
    map.remove("existing_key");

    let entry = map
        .raw_entry_mut()
        .from_key("existing_key")
        .and_replace_entry_with(|_k, _v| Some(5));

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

