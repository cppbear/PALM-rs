// Answer 0

#[test]
fn test_hashset_new() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let set: HashSet<i32, DefaultHashBuilder, DummyAllocator> = HashSet::new();
    assert_eq!(set.map.table.len(), 0);
}

#[test]
fn test_hashset_with_capacity() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let set: HashSet<i32, DefaultHashBuilder, DummyAllocator> = HashSet::with_capacity(10);
    assert_eq!(set.map.table.len(), 0);
}

