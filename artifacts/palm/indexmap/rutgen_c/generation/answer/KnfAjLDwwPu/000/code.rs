// Answer 0

#[test]
fn test_get_key_value_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::with_hasher(TestHasher);
    map.insert(1, "one");
    map.insert(2, "two");

    assert_eq!(map.get_key_value(&1), Some((&1, &"one")));
    assert_eq!(map.get_key_value(&2), Some((&2, &"two")));
}

#[test]
fn test_get_key_value_not_found() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::with_hasher(TestHasher);
    map.insert(1, "one");

    assert_eq!(map.get_key_value(&2), None);
}

#[test]
fn test_get_key_value_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let map: IndexMap<i32, &str, TestHasher> = IndexMap::with_hasher(TestHasher);

    assert_eq!(map.get_key_value(&1), None);
}

