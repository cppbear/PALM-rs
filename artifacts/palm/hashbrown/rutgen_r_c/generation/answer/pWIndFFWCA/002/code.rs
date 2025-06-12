// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("poneyland".to_owned(), 3);

    let entry = map.entry_ref("poneyland");
    let value_ref = entry.or_insert(10);
    assert_eq!(*value_ref, 3); // Should return the existing value

    *value_ref *= 2; // Modify the value
    assert_eq!(map["poneyland"], 6); // Check the modified value
}

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<String, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();

    // Use or_insert on a vacant entry
    let entry = map.entry_ref("not_found");
    let value_ref = entry.or_insert(7);
    assert_eq!(*value_ref, 7); // Should insert the default value

    // Ensure it can be modified as well
    *value_ref += 3; // Modify the inserted value
    assert_eq!(map["not_found"], 10); // Check the modified value
}

