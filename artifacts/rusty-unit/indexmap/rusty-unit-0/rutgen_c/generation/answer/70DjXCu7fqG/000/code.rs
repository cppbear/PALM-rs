// Answer 0

#[test]
fn test_get_index_of_empty_map() {
    struct TestHashBuilder;
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }
    
    let map: IndexMap<i32, &str, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };

    assert_eq!(map.get_index_of(&1), None);
}

#[test]
fn test_get_index_of_single_entry() {
    struct TestHashBuilder;
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };

    map.core.entries.push(Bucket { hash: HashValue(1), key: 1, value: "one" });
    
    assert_eq!(map.get_index_of(&1), Some(0));
    assert_eq!(map.get_index_of(&2), None);
}

#[test]
fn test_get_index_of_multiple_entries() {
    struct TestHashBuilder;
    impl BuildHasher for TestHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };

    map.core.entries.push(Bucket { hash: HashValue(1), key: 1, value: "one" });
    map.core.entries.push(Bucket { hash: HashValue(2), key: 2, value: "two" });

    assert_eq!(map.get_index_of(&1), Some(0));
    assert_eq!(map.get_index_of(&2), Some(1));
    assert_eq!(map.get_index_of(&3), None);
}

