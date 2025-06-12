// Answer 0

#[test]
fn test_raw_entry_v1_valid() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let map: IndexMap<i32, i32, DummyHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    let raw_entry = map.raw_entry_v1();
    assert_eq!(raw_entry.map, &map);
}

#[test]
#[should_panic]
fn test_raw_entry_v1_invalid() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, DummyHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: DummyHasher,
    };

    let raw_entry = map.raw_entry_mut_v1(); // This method is assumed to be a potential panic triggering point.
    assert_eq!(raw_entry.map, &map); // This line should not be reached.
}

