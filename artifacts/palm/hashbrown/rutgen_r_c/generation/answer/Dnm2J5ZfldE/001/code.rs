// Answer 0

#[test]
fn test_allocator_default() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::slice_from_raw_parts_mut(core::ptr::null_mut(), layout.size()))
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }
    
    let hasher = DefaultHashBuilder::new();
    let allocator = TestAllocator;
    let hash_set = HashSet::<i32, _, TestAllocator>::with_hasher_in(hasher, allocator);
    
    let alloc = hash_set.allocator();
    assert!(alloc as *const _ == &hash_set.map.allocator() as *const _);
}

#[test]
fn test_allocator_with_capacity() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::slice_from_raw_parts_mut(core::ptr::null_mut(), layout.size()))
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }
    
    let hasher = DefaultHashBuilder::new();
    let allocator = TestAllocator;
    let hash_set = HashSet::<i32, _, TestAllocator>::with_capacity_and_hasher_in(10, hasher, allocator);
    
    let alloc = hash_set.allocator();
    assert!(alloc as *const _ == &hash_set.map.allocator() as *const _);
}

