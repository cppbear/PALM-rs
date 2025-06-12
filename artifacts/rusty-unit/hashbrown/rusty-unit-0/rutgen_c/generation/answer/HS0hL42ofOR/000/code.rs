// Answer 0

#[test]
fn test_hashset_with_capacity_nonzero() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let set: HashSet<i32, DefaultHashBuilder, TestAllocator> = HashSet::with_capacity_in(10, alloc);
    assert!(set.map.table.len() >= 10);
}

#[test]
fn test_hashset_with_capacity_zero() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let set: HashSet<i32, DefaultHashBuilder, TestAllocator> = HashSet::with_capacity_in(0, alloc);
    assert!(set.map.table.len() == 0);
}

