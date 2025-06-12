// Answer 0

#[test]
fn test_contains_key_existing_key() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    assert!(map.contains_key(&1));
}

#[test]
fn test_contains_key_non_existing_key() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    assert!(!map.contains_key(&3));
}

#[test]
fn test_contains_key_empty_map() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let map: IndexMap<i32, &str, TestHasher> = IndexMap::new();

    assert!(!map.contains_key(&1));
}

#[test]
fn test_contains_key_with_equivalent() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    #[derive(Hash, Eq, PartialEq)]
    struct TestKey(i32);

    let mut map: IndexMap<TestKey, &str, TestHasher> = IndexMap::new();
    map.insert(TestKey(1), "one");
    map.insert(TestKey(2), "two");

    assert!(map.contains_key(&TestKey(1)));
    assert!(!map.contains_key(&TestKey(3)));
}

