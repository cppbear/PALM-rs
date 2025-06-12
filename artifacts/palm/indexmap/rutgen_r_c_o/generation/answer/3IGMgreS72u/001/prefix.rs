// Answer 0

#[test]
fn test_and_modify_vacant_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::DummyHasher
        }
    }

    let mut entries = Entries::<String, i32>::new();
    let hasher = TestHasher;
    
    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut entries),
        hash_builder: &hasher,
    });

    let f = |k: &mut String, v: &mut i32| {
        // No modification
    };

    let _result = vacant_entry.and_modify(f);
}

#[test]
fn test_and_modify_non_hashable_key() {
    struct NonHashable;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::hash::DummyHasher
        }
    }

    let mut entries = Entries::<NonHashable, i32>::new();
    let hasher = TestHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(&mut entries),
        hash_builder: &hasher,
    });

    let f = |k: &mut NonHashable, v: &mut i32| {
        // No modification
    };

    let _result = vacant_entry.and_modify(f);
}

