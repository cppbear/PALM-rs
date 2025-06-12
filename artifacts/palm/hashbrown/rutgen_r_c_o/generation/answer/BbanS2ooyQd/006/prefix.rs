// Answer 0

#[test]
fn test_shrink_to_with_min_size_zero_and_no_capacities() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<u32, TestAllocator>::new_in(TestAllocator);
    table.shrink_to(0, |x| *x);
}

#[test]
fn test_shrink_to_with_min_size_zero_and_capacity_between_1_and_7() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::<u32, TestAllocator>::with_capacity_in(5, TestAllocator);
    table.shrink_to(0, |x| *x);
}

