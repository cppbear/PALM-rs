// Answer 0

#[test]
fn test_get_mut_key_not_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();

    map.insert(1, "One".to_string());
    map.insert(2, "Two".to_string());

    let result = map.get_mut(&3);
}

#[test]
fn test_get_mut_key_not_found_negative() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();

    map.insert(10, "Ten".to_string());
    map.insert(20, "Twenty".to_string());

    let result = map.get_mut(&15);
}

#[test]
fn test_get_mut_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();

    let result = map.get_mut(&1);
}

