// Answer 0

#[test]
fn test_hasher() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    let hasher = DummyHasher;
    
    let index_set: super::IndexSet<i32, DummyHasher> = super::IndexSet::with_hasher(hasher);
    
    assert_eq!(index_set.hasher() as *const _, &hasher as *const _);
}

#[test]
fn test_hasher_empty() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    let hasher = DummyHasher;
    
    let index_set: super::IndexSet<i32, DummyHasher> = super::IndexSet::with_capacity_and_hasher(0, hasher);
    
    assert_eq!(index_set.hasher() as *const _, &hasher as *const _);
}

