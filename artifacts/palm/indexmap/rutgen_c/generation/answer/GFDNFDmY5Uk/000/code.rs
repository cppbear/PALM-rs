// Answer 0

#[test]
fn test_is_empty_when_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::BuiltHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::BuiltHasher::default()
        }
    }

    let map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap::with_hasher(TestHasher);
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_when_not_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::BuiltHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::BuiltHasher::default()
        }
    }

    let mut map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap::with_hasher(TestHasher);
    map.core.entries.push((1, 100)); // Using any method available to add an entry
    assert!(!map.is_empty());
}

