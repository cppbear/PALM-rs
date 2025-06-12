// Answer 0

#[test]
fn test_or_default_vacant_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, Option<u32>, BuildHasherDefault<DefaultHasher>, TestAllocator> = HashMap::new();
    let entry = Entry::Vacant(VacantEntry {
        hash: 0,
        key: "poneyland",
        table: &mut map,
    });

    let result = entry.or_default();
    assert_eq!(*result, None);
}

#[test]
fn test_or_default_occupied_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, Option<u32>, BuildHasherDefault<DefaultHasher>, TestAllocator> = HashMap::new();
    map.insert("horseland", Some(3));

    let entry = Entry::Occupied(OccupiedEntry {
        hash: 1,
        elem: map.get_bucket("horseland").unwrap(),
        table: &mut map,
    });

    let result = entry.or_default();
    assert_eq!(*result, Some(3));
}

#[test]
#[should_panic]
fn test_or_default_panic_empty_map() {
    use hashbrown::HashMap;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let map: HashMap<&str, Option<u32>, BuildHasherDefault<DefaultHasher>, TestAllocator> = HashMap::new();
    let entry = Entry::Vacant(VacantEntry {
        hash: 0,
        key: "nonexistent",
        table: &mut map,
    });

    // This should panic because `nonexistent` key is not in the map.
    let _result = entry.or_default();
}

