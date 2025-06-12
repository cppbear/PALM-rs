// Answer 0

#[test]
fn test_is_empty_on_empty_set() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::fnv::Fnv64Hasher;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let set: super::IndexSet<i32, DummyHasher> = super::IndexSet::with_capacity_and_hasher(0, DummyHasher);
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_on_non_empty_set() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::fnv::Fnv64Hasher;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut set: super::IndexSet<i32, DummyHasher> = super::IndexSet::with_capacity_and_hasher(0, DummyHasher);
    set.map.insert(1, ()); // Assuming there's some method to insert elements
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::fnv::Fnv64Hasher;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut set: super::IndexSet<i32, DummyHasher> = super::IndexSet::with_capacity_and_hasher(0, DummyHasher);
    set.map.insert(1, ()); // Assuming there's some method to insert elements
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_after_truncate() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::fnv::Fnv64Hasher;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut set: super::IndexSet<i32, DummyHasher> = super::IndexSet::with_capacity_and_hasher(0, DummyHasher);
    set.map.insert(1, ()); // Assuming there's some method to insert elements
    set.truncate(0);
    assert!(set.is_empty());
}

