// Answer 0

#[test]
fn test_insert_existing_key() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, SimpleHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: SimpleHasher,
    };
    
    index_map.insert(1, "Value1".to_string());
    let old_value = index_map.insert(1, "Value2".to_string());
    assert_eq!(old_value, Some("Value1".to_string()));
}

#[test]
fn test_insert_new_key() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, SimpleHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: SimpleHasher,
    };

    let old_value = index_map.insert(2, "Value2".to_string());
    assert_eq!(old_value, None);
}

#[test]
fn test_insert_multiple_keys() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, SimpleHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: SimpleHasher,
    };

    index_map.insert(3, "Value3".to_string());
    index_map.insert(3, "Value4".to_string());
    assert_eq!(index_map.insert(4, "Value5".to_string()), None);
    assert_eq!(index_map.insert(3, "Value6".to_string()), Some("Value4".to_string()));
}

#[test]
#[should_panic]
fn test_insert_panic_out_of_bounds() {
    struct SimpleHasher;
    impl BuildHasher for SimpleHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, SimpleHasher> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: SimpleHasher,
    };

    // Attempt to create an out of bounds entry.
    index_map.insert(std::i32::MAX, "ValueOutOfBounds".to_string());
}

