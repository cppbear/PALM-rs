// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, String> = HashMap::new();
    let hash_builder = BuildHasherDefault::<DefaultHasher>::default();
    let mut table = RawTable::new(); // Assuming RawTable has a new method

    let entry = RawEntryMut::Vacant(RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hash_builder,
    });

    let (key, value) = entry.or_insert_with(|| ("poneyland", "hoho".to_string()));

    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::HashMap;
    use std::hash::BuildHasherDefault;
    use std::collections::hash_map::DefaultHasher;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("poneyland", "hoho".to_string());

    let hash_builder = BuildHasherDefault::<DefaultHasher>::default();
    let mut table = RawTable::new(); // Assuming RawTable has a new method

    let entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        elem: Bucket::new(), // Assuming Bucket has a method to create an instance
        table: &mut table,
        hash_builder: &hash_builder,
    });

    let (key, value) = entry.or_insert_with(|| ("another_key", "another_value".to_string()));

    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
}

