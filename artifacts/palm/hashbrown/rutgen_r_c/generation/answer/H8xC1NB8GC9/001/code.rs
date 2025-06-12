// Answer 0

#[test]
fn test_remove_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    // Helper struct for testing
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    // Instantiate HashMap with custom allocator
    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();

    // Verify initial state of the map
    assert!(map.is_empty());

    // Insert a key-value pair
    map.insert("poneyland", 12);

    // Check the occupied entry and remove it
    if let Entry::Occupied(o) = map.entry("poneyland") {
        assert_eq!(o.remove_entry(), ("poneyland", 12));
    }

    // Check if the map is now empty after removal
    assert!(map.is_empty());
    assert_eq!(map.contains_key("poneyland"), false);
}

#[test]
#[should_panic]
fn test_remove_entry_non_existent() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();

    // Attempt to remove an entry that doesn't exist
    if let Entry::Occupied(o) = map.entry("not_found") {
        // This is expected to panic since the entry does not exist
        let _ = o.remove_entry();
    }
}

