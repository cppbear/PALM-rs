// Answer 0

#[test]
fn test_and_replace_entry_with_vacant() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::RawEntryMut;

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry = map
        .raw_entry_mut()
        .from_key("non_existent_key")
        .and_replace_entry_with(|_k, _v| panic!());

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_after_insertion() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::RawEntryMut;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    map.insert("existing_key", 100);

    let entry = map
        .raw_entry_mut()
        .from_key("non_existent_key")
        .and_replace_entry_with(|_k, _v| panic!());

    match entry {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

#[test]
fn test_and_replace_entry_with_vacant_multiple_calls() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::RawEntryMut;

    let mut map: HashMap<&str, u32> = HashMap::new();

    let entry1 = map
        .raw_entry_mut()
        .from_key("non_existent_key_1")
        .and_replace_entry_with(|_k, _v| panic!());

    match entry1 {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }

    let entry2 = map
        .raw_entry_mut()
        .from_key("non_existent_key_2")
        .and_replace_entry_with(|_k, _v| panic!());

    match entry2 {
        RawEntryMut::Vacant(_) => {},
        RawEntryMut::Occupied(_) => panic!(),
    }
}

