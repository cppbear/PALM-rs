// Answer 0

#[test]
fn test_is_empty_when_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let set: super::IndexSet<i32, TestHasher> = super::IndexSet::with_hasher(TestHasher);
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_when_not_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, TestHasher> = super::IndexSet::with_hasher(TestHasher);
    set.map.insert(1, ()); // Assuming insert is a valid method to add elements to the map
    assert!(!set.is_empty());
}

