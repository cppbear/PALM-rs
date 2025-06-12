// Answer 0

#[test]
fn test_raw_entry_mut_debug_occupied() {
    use hashbrown::HashMap;

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut entries: Entries<usize, &str> = Entries::new();
    let hash_builder = TestHashBuilder;

    let mut map: IndexMap<usize, &str, TestHashBuilder> = IndexMap::with_hasher(hash_builder);

    map.insert(1, "Occupied Entry");
    let occupied_index = map.get_index_of(&1).unwrap();

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(occupied_index, &mut map),
        hash_builder: PhantomData,
    };

    let raw_entry_mut = RawEntryMut::Occupied(occupied_entry);
    
    let mut output = String::new();
    let result = raw_entry_mut.fmt(&mut output).unwrap();

    assert!(result.is_ok());
    assert!(output.contains("RawEntryMut"));
    assert!(output.contains("Occupied"));
}

#[test]
fn test_raw_entry_mut_debug_vacant() {
    use hashbrown::HashMap;

    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut entries: Entries<usize, &str> = Entries::new();
    let hash_builder = TestHashBuilder;

    let map: IndexMap<usize, &str, TestHashBuilder> = IndexMap::with_hasher(hash_builder);

    let vacant_entry = RawVacantEntryMut {
        map: RefMut::new(&map),
        hash_builder: &hash_builder,
    };

    let raw_entry_mut = RawEntryMut::Vacant(vacant_entry);
    
    let mut output = String::new();
    let result = raw_entry_mut.fmt(&mut output).unwrap();

    assert!(result.is_ok());
    assert!(output.contains("RawEntryMut"));
    assert!(output.contains("Vacant"));
}

