// Answer 0

#[test]
fn test_insert_hashed_nocheck() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::fnv::FnvHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::fnv::FnvHasher::default()
        }
    }

    let mut indices = vec![];
    let mut entries = vec![];
    let hasher = DummyHasher;
    let hash_builder = &hasher;

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder,
    };

    let (key_ref, value_ref) = vacant_entry
        .insert_hashed_nocheck(42, "test_key", "test_value");

    assert_eq!(*key_ref, "test_key");
    assert_eq!(*value_ref, "test_value");
}

#[test]
fn test_insert_hashed_nocheck_with_different_data() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::hash::fnv::FnvHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::fnv::FnvHasher::default()
        }
    }

    let mut indices = vec![];
    let mut entries = vec![];
    let hasher = DummyHasher;
    let hash_builder = &hasher;

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder,
    };

    let (key_ref, value_ref) = vacant_entry
        .insert_hashed_nocheck(84, "another_key", "another_value");

    assert_eq!(*key_ref, "another_key");
    assert_eq!(*value_ref, "another_value");
}

