// Answer 0

#[test]
fn test_with_hasher_in_default_builder() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let map = HashMap::with_hasher_in(hash_builder, alloc);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_with_hasher_in_and_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = CustomAllocator;
    let hash_builder = DefaultHashBuilder::default();
    let map = HashMap::with_hasher_in(hash_builder, alloc);
    assert_eq!(map.capacity(), 0);
}

