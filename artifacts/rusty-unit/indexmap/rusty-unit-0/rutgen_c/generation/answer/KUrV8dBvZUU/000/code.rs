// Answer 0

#[test]
fn test_entry_insertion_vacant() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, DummyHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    let entry = map.entry(42);
    match entry {
        Entry::Vacant(v) => {
            let _ = v.insert("value");
            assert_eq!(map.core.entries.as_entries().len(), 1);
        },
        _ => panic!("Expected Vacant entry"),
    }
}

#[test]
fn test_entry_insertion_occupied() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, DummyHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    map.insert(42, "original");
    let entry = map.entry(42);
    match entry {
        Entry::Occupied(o) => {
            let old_value = o.insert("new_value");
            assert_eq!(old_value, Some("original"));
        },
        _ => panic!("Expected Occupied entry"),
    }
}

#[test]
fn test_entry_non_existent_key() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, DummyHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    let entry = map.entry(99);
    match entry {
        Entry::Vacant(v) => {
            let _ = v.insert("new_entry");
            assert_eq!(map.core.entries.as_entries().len(), 1);
        },
        _ => panic!("Expected Vacant entry"),
    }
}

