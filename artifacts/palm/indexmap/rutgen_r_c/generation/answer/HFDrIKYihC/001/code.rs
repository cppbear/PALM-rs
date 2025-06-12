// Answer 0

#[test]
fn test_index_vacant_entry() {
    struct DummyKey;
    struct DummyValue;

    impl Default for DummyValue {
        fn default() -> Self {
            DummyValue
        }
    }

    struct DummyEntries<K, V> {
        indices: Vec<usize>,
        _marker: std::marker::PhantomData<(K, V)>,
    }

    impl<K, V> DummyEntries<K, V> {
        fn new() -> Self {
            Self {
                indices: Vec::new(),
                _marker: std::marker::PhantomData,
            }
        }
    }

    let dummy_entries = DummyEntries::<DummyKey, DummyValue>::new();
    let vacant_entry = super::VacantEntry {
        map: super::RefMut::new(&mut dummy_entries),
        hash: crate::HashValue::default(),
        key: DummyKey,
    };
    let entry = super::Entry::Vacant(vacant_entry);

    assert_eq!(entry.index(), 0);
}

