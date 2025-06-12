// Answer 0

#[test]
fn test_insert_on_vacant_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::collections::hash_map::DefaultHasher;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let hasher = DefaultHasher::new();
    
    // Call to insert on a vacant entry
    let entry = map.raw_entry_mut().from_key("vacant_key");
    let occupied_entry = entry.insert("vacant_key", 42);

    // Verify the inserted key-value pair is correct
    assert_eq!(occupied_entry.remove_entry(), ("vacant_key", 42));
}

#[test]
fn test_insert_on_occupied_entry() {
    use hashbrown::{hash_map::RawEntryMut, HashMap};
    use std::collections::hash_map::DefaultHasher;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let hasher = DefaultHasher::new();

    // First insert that will create an occupied entry
    map.insert("occupied_key", 100);
    
    // Call to insert on an occupied entry
    let entry = map.raw_entry_mut().from_key("occupied_key");
    let occupied_entry = entry.insert("occupied_key", 200);

    // Verify the updated key-value pair is correct
    assert_eq!(occupied_entry.remove_entry(), ("occupied_key", 200));
}

