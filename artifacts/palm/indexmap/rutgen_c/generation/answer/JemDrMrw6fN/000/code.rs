// Answer 0

#[test]
fn test_or_insert_with_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![] }
        }
    }

    let mut hasher = DefaultHasher::new();
    let mut test_map = TestMap::new();
    let hash_builder: &DefaultHasher = &hasher;

    struct Entries<'a, K, V> {
        map: &'a mut TestMap,
    }

    let mut entries = Entries { map: &mut test_map };

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut { entries: &mut entries },
        hash_builder,
    });

    let (key, value) = vacant_entry.or_insert_with(|| (1, 2));
    assert_eq!(*key, 1);
    assert_eq!(*value, 2);
}

#[test]
fn test_or_insert_with_occupied() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct TestMap {
        entries: Vec<(usize, usize)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: vec![(1, 2)] }
        }
    }

    let mut hasher = DefaultHasher::new();
    let hash_builder: &DefaultHasher = &hasher;

    let mut test_map = TestMap::new();
    let entries = Entries { map: &mut test_map };

    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(0),
        hash_builder: PhantomData,
    });

    let (key, value) = occupied_entry.or_insert_with(|| (3, 4));
    assert_eq!(*key, 1);
    assert_eq!(*value, 2);
}

