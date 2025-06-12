// Answer 0

#[test]
fn test_get_mut() {
    use crate::raw::Global;
    use crate::hashbrown::HashMap;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("poneyland", 12);

    // Verify initial value
    assert_eq!(map.get("poneyland"), Some(&12));

    // Wrap the occupied entry retrieval and mutation in a safe context
    if let Some(entry) = map.get_mut("poneyland") {
        *entry += 10;
        assert_eq!(*entry, 22);

        // We can use the same entry multiple times.
        *entry += 2;
    }

    // Validate final value
    assert_eq!(map.get("poneyland"), Some(&24));
}

#[test]
#[should_panic]
fn test_get_mut_empty_entry() {
    use crate::raw::Global;
    use crate::hashbrown::HashMap;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();

    // Attempt to get_mut on a non-existent entry should panic
    let _ = map.get_mut("nonexistent"); // This should trigger panic
}

