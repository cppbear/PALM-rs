// Answer 0

#[test]
fn test_raw_entry_mut_index_occupied() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct DummyMap<K, V, S> {
        entries: Vec<(K, V)>,
        hasher: S,
    }

    impl<K, V, S> DummyMap<K, V, S> {
        fn new(hasher: S) -> Self {
            Self {
                entries: Vec::new(),
                hasher,
            }
        }
    }

    let mut entries = DummyMap::new(DummyHasher);
    entries.entries.push((1, "value1"));

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries.entries,
        index: hashbrown::hash_table::OccupiedEntry::from_mut(0, entries.entries.as_mut_slice()),
        hash_builder: PhantomData,
    };

    let raw_entry = RawEntryMut::Occupied(occupied_entry);

    assert_eq!(raw_entry.index(), 0);
}

#[test]
fn test_raw_entry_mut_index_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct DummyHasher;

    impl BuildHasher for DummyHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct DummyMap<K, V, S> {
        entries: Vec<(K, V)>,
        hasher: S,
    }

    impl<K, V, S> DummyMap<K, V, S> {
        fn new(hasher: S) -> Self {
            Self {
                entries: Vec::new(),
                hasher,
            }
        }
    }

    let entries = DummyMap::new(DummyHasher);
    let vacant_entry = RawVacantEntryMut {
        map: RefMut { map: &entries },
        hash_builder: &DummyHasher,
    };

    let raw_entry = RawEntryMut::Vacant(vacant_entry);

    assert_eq!(raw_entry.index(), 0);
}

