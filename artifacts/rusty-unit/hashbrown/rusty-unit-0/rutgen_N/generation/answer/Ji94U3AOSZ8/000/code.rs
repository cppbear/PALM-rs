// Answer 0

#[test]
fn test_insert_entry_vacant() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::RandomState;

    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Result = u64;
        fn build_hasher(&self) -> std::collections::hash_map::RandomState {
            RandomState::new()
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    if let EntryRef::Vacant(v) = map.entry_ref("poneyland") {
        let o = v.insert_entry(37);
        assert_eq!(o.get(), &37);
    } else {
        panic!("Expected a VacantEntryRef");
    }
}

#[test]
fn test_insert_entry_overwrite_existing() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let EntryRef::Vacant(v) = map.entry_ref("poneyland") {
        let o = v.insert_entry(37);
        assert_eq!(o.get(), &37);
    }

    if let EntryRef::Occupied(o) = map.entry_ref("poneyland") {
        let new_value = o.insert_entry(50);
        assert_eq!(new_value.get(), &50);
    } else {
        panic!("Expected an OccupiedEntryRef");
    }
}

#[test]
#[should_panic]
fn test_insert_entry_non_existent_vacant() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::EntryRef;

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let EntryRef::Occupied(_) = map.entry_ref("nonexistent") {
        panic!("Expected a VacantEntryRef");
    }
}

