// Answer 0

#[test]
fn test_or_insert_with_occupied() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;
    use std::ptr;

    struct TestHasher(DefaultHasher);
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    struct DummyMap {
        entries: Vec<(i32, String)>,
    }

    struct DummyEntries<'a> {
        map: &'a mut DummyMap,
    }

    impl<'a> DummyEntries<'a> {
        fn get_key_value_mut(&mut self, index: usize) -> (&'a mut i32, &'a mut String) {
            let (key, value) = &mut self.map.entries[index];
            (key, value)
        }
    }

    let mut map = DummyMap { entries: vec![(1, "value1".to_string()), (2, "value2".to_string())] };
    let mut entries = DummyEntries { map: &mut map };

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: 0,
        hash_builder: PhantomData::<&TestHasher>(),
    };

    let entry = RawEntryMut::Occupied(occupied_entry);

    let result = entry.or_insert_with(|| (3, "value3".to_string()));

    assert_eq!(*result.0, 1);
    assert_eq!(*result.1, "value1");
}

#[test]
fn test_or_insert_with_vacant() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct TestHasher(DefaultHasher);
    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    struct DummyMap {
        entries: Vec<(i32, String)>,
    }

    struct DummyEntries<'a> {
        map: &'a mut DummyMap,
    }

    impl<'a> DummyEntries<'a> {
        fn get_key_value_mut(&mut self, index: usize) -> (&'a mut i32, &'a mut String) {
            let (key, value) = &mut self.map.entries[index];
            (key, value)
        }
    }

    let mut map = DummyMap { entries: vec![] };
    let mut entries = DummyEntries { map: &mut map };
    let vacant_entry = RawVacantEntryMut {
        map: RefMut { data: ptr::null_mut() }, 
        hash_builder: &TestHasher(DefaultHasher::new()),
    };

    let entry = RawEntryMut::Vacant(vacant_entry);

    let result = entry.or_insert_with(|| (3, "value3".to_string()));

    assert_eq!(*result.0, 3);
    assert_eq!(*result.1, "value3");
}

