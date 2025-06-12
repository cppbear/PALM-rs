// Answer 0

#[test]
fn test_raw_occupied_entry_mut_fmt_debug_with_valid_data() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut entries = IndexMap::<i32, String, TestHashBuilder>::new();
    entries.insert(1, "one".to_string());
    entries.insert(2, "two".to_string());
    entries.insert(3, "three".to_string());

    let occupied_entry = entries.get_mut(&2).unwrap();
    let index = occupied_entry.index();

    let mut raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: occupied_entry,
        hash_builder: PhantomData,
    };

    let _ = fmt::Debug::fmt(&raw_entry, &mut fmt::Formatter::new());
}

#[test]
fn test_raw_occupied_entry_mut_fmt_debug_with_empty_entries() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut entries: IndexMap<i32, String, TestHashBuilder> = IndexMap::new();

    if let Some(occupied_entry) = entries.get_mut(&0) {
        let index = occupied_entry.index();

        let mut raw_entry = RawOccupiedEntryMut {
            entries: &mut entries,
            index: occupied_entry,
            hash_builder: PhantomData,
        };

        let _ = fmt::Debug::fmt(&raw_entry, &mut fmt::Formatter::new());
    }
}

#[test]
#[should_panic]
fn test_raw_occupied_entry_mut_fmt_debug_with_invalid_index() {
    struct TestHashBuilder;

    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut entries = IndexMap::<i32, String, TestHashBuilder>::new();
    entries.insert(1, "one".to_string());

    let occupied_entry = entries.get_mut(&1).unwrap();
    let index = occupied_entry.index();

    // Directly create a RawOccupiedEntryMut with an invalid index
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_raw(index),
        hash_builder: PhantomData,
    };

    let _ = fmt::Debug::fmt(&raw_entry, &mut fmt::Formatter::new());
}

