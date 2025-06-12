// Answer 0

#[test]
fn test_raw_vacant_entry_mut_debug_fmt() {
    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = core::hash::SipHasher13;

        fn build_hasher(&self) -> Self::Hasher {
            core::hash::SipHasher13::new()
        }
    }

    struct Indices;

    struct Entries<K, V> {
        _marker: PhantomData<(K, V)>,
    }

    let indices = &mut Indices;
    let entries = &mut Entries { _marker: PhantomData };
    let ref_mut = RefMut { indices, entries };
    let hasher = DummyHasher;
    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &hasher,
    };

    let mut output = vec![];
    let result = std::fmt::write(&mut output, format_args!("{:?}", raw_entry));

    assert!(result.is_ok());
    assert!(!output.is_empty());
}

