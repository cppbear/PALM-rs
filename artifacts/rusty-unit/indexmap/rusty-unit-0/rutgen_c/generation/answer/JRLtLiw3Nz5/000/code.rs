// Answer 0

#[test]
fn test_insert_new_key() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };

    let result = map.insert(1, 100);
    assert_eq!(result, None);
    assert_eq!(map.core.entries.as_entries()[0], (1, 100));
}

#[test]
fn test_insert_existing_key() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };

    map.insert(1, 100);
    let result = map.insert(1, 200);
    assert_eq!(result, Some(100));
    assert_eq!(map.core.entries.as_entries()[0], (1, 200));
}

#[test]
fn test_insert_multiple_keys() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::hash::rustc_hash::FxHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::rustc_hash::FxHasher::default()
        }
    }

    let mut map: IndexMap<i32, i32, HashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: HashBuilder,
    };

    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(1, 150);

    assert_eq!(map.core.entries.as_entries().len(), 2);
    assert_eq!(map.core.entries.as_entries()[0], (1, 150));
    assert_eq!(map.core.entries.as_entries()[1], (2, 200));
}

