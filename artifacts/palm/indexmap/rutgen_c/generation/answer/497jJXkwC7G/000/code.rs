// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct SimpleBuilder;
    impl BuildHasher for SimpleBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut entries = Entries::new();
    let key: usize = 1;
    let value: String = "default".to_string();
    let mut index_map: IndexMap<usize, String, SimpleBuilder> = IndexMap::new();

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::from(&mut index_map),
        hash_builder: &SimpleBuilder,
    });

    let (key_ref, value_ref) = vacant_entry.or_insert(key, value.clone());
    assert_eq!(*key_ref, key);
    assert_eq!(*value_ref, value);
}

#[test]
fn test_or_insert_occupied_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct SimpleBuilder;
    impl BuildHasher for SimpleBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut entries = Entries::new();
    let key: usize = 1;
    let value: String = "existing".to_string();
    let mut index_map: IndexMap<usize, String, SimpleBuilder> = IndexMap::new();
    index_map.insert(key, value.clone());

    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut entries,
        index: index_map.get_key_value(&key).unwrap().1,
        hash_builder: PhantomData,
    });

    let (key_ref, value_ref) = occupied_entry.or_insert(key, "new_value".to_string());
    assert_eq!(*key_ref, key);
    assert_eq!(*value_ref, value);
}

