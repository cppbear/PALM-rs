// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;
    
    struct CustomHasher;
    
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
    
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
    
    let mut map: IndexMap<i32, String, BuildHasherDefault<CustomHasher>> = IndexMap::new();
    map.insert(1, "one".to_string());
    
    let entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries(),
        index: hashbrown::hash_table::OccupiedEntry::occupy(&mut map, &1),
        hash_builder: PhantomData,
    });
    
    let modified_entry = entry.and_modify(|k, v| {
        *k += 1; // increment key
        v.push_str(" modified"); // modify value
    });
    
    if let RawEntryMut::Occupied(entry) = modified_entry {
        let (k, v) = entry.get_key_value_mut();
        assert_eq!(*k, 2);
        assert_eq!(v, "one modified");
    }
}

#[test]
fn test_and_modify_vacant_entry() {
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;
    
    struct CustomHasher;
    
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
    
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }
    
    let mut map: IndexMap<i32, String, BuildHasherDefault<CustomHasher>> = IndexMap::new();
    
    let entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut map),
        hash_builder: &CustomHasher,
    });
    
    let modified_entry = entry.and_modify(|_k, _v| {
        // This closure should not be executed as the slot is vacant
        panic!("This should not be called");
    });

    // Ensure that we still have a vacant entry and did not panic
    if let RawEntryMut::Vacant(_) = modified_entry {
        // Successfully handled the vacant entry
    }
}

