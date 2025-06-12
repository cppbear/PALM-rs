// Answer 0

#[test]
fn test_raw_entry_mut_index_occupied() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let mut map: IndexMap<u32, u32, TestHasher> = IndexMap::new();
    
    map.insert(1, 100);
    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.entries.index_of(&1).unwrap(),
        hash_builder: PhantomData,
    });

    let index_value = occupied_entry.index();
}

#[test]
fn test_raw_entry_mut_index_occupied_with_multiple_entries() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let mut map: IndexMap<u32, u32, TestHasher> = IndexMap::new();

    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.entries.index_of(&2).unwrap(),
        hash_builder: PhantomData,
    });

    let index_value = occupied_entry.index();
}

#[test]
fn test_raw_entry_mut_index_occupied_at_boundary() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let mut map: IndexMap<u32, u32, TestHasher> = IndexMap::new();

    map.insert(0, 0);
    
    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.entries.index_of(&0).unwrap(),
        hash_builder: PhantomData,
    });

    let index_value = occupied_entry.index();
}

#[test]
fn test_raw_entry_mut_index_multiple_insertions() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = TestHasher;
    let mut map: IndexMap<u32, u32, TestHasher> = IndexMap::new();

    map.insert(3, 300);
    map.insert(7, 700);
    
    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.entries.index_of(&3).unwrap(),
        hash_builder: PhantomData,
    });

    let index_value = occupied_entry.index();
}

