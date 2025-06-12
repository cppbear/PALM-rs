// Answer 0

#[test]
fn test_or_insert_with_vacant_case() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    // Helper structures for testing
    struct TestHasher(BuildHasherDefault<DefaultHasher>);
    
    impl Default for TestHasher {
        fn default() -> Self {
            TestHasher(DefaultHasher::default())
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::default();
    
    // Create a Vacant entry
    let vacant_entry = RawEntryMut::Vacant(
        RawVacantEntryMut {
            map: RefMut::new(&mut map),
            hash_builder: &TestHasher::default(),
        }
    );

    // Callable closure that generates key-value pair
    let value_pair = || (10, 20);

    // Call or_insert_with on the vacant entry
    let (key_ref, value_ref) = vacant_entry.or_insert_with(value_pair);
    
    assert_eq!(*key_ref, 10);
    assert_eq!(*value_ref, 20);
}

#[test]
fn test_or_insert_with_occupied_case() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    // Helper structures for testing
    struct TestHasher(BuildHasherDefault<DefaultHasher>);
    
    impl Default for TestHasher {
        fn default() -> Self {
            TestHasher(DefaultHasher::default())
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::default();
    map.insert(10, 20);
    
    // Create an Occupied entry
    let occupied_entry = RawEntryMut::Occupied(
        RawOccupiedEntryMut {
            entries: &mut map.entries(),
            index: map.get_key_value_index(&10).unwrap(),
            hash_builder: PhantomData,
        }
    );

    // Callable closure that generates key-value pair
    let value_pair = || (30, 40);

    // Call or_insert_with on the occupied entry
    let (key_ref, value_ref) = occupied_entry.or_insert_with(value_pair);
    
    assert_eq!(*key_ref, 10);
    assert_eq!(*value_ref, 20);
}

