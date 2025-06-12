// Answer 0

#[test]
fn test_raw_entry_mut_debug_vacant() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut table = crate::raw::RawTable::<(&str, i32), TestAllocator>::new();
    let hash_builder = std::collections::hash_map::RandomState::new();
    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        table: &mut table,
        hash_builder: &hash_builder,
    });
    
    let debug_output = format!("{:?}", vacant_entry);
    assert!(debug_output.contains("RawEntry"));
}

#[test]
fn test_raw_entry_mut_debug_occupied() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut table = crate::raw::RawTable::<(&str, i32), TestAllocator>::new();
    let hash_builder = std::collections::hash_map::RandomState::new();
    let occupied_entry = RawEntryMut::Occupied(RawOccupiedEntryMut {
        elem: Bucket((String::from("key"), 42)),
        table: &mut table,
        hash_builder: &hash_builder,
    });
    
    let debug_output = format!("{:?}", occupied_entry);
    assert!(debug_output.contains("RawEntry"));
}

