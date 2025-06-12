// Answer 0

#[test]
fn test_hashmap_with_hasher_in_default() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            todo!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }
    
    let allocator = TestAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_hasher_in(hash_builder, allocator);
    
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_hashmap_with_hasher_in_capacity() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            todo!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let allocator = TestAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(10, hash_builder, allocator);
    
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

