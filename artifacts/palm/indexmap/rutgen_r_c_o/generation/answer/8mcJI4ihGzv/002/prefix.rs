// Answer 0

#[test]
fn test_shift_remove_entry_key_not_present() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);

    let result = map.shift_remove_entry(&4);
}

#[test]
fn test_shift_remove_entry_with_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::new();

    let result = map.shift_remove_entry(&1);
}

