// Answer 0

#[test]
fn test_with_capacity_and_hasher_in_non_zero_capacity() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {}
    }

    let hash_builder = DefaultHashBuilder::default();
    let allocator = TestAllocator;
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(10, hash_builder, allocator);

    assert_eq!(map.capacity(), 10);
    assert!(map.is_empty());
}

#[test]
fn test_with_capacity_and_hasher_in_zero_capacity() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {}
    }

    let hash_builder = DefaultHashBuilder::default();
    let allocator = TestAllocator;
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> =
        HashMap::with_capacity_and_hasher_in(0, hash_builder, allocator);

    assert_eq!(map.capacity(), 0);
    assert!(map.is_empty());
}

