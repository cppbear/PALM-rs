// Answer 0

#[test]
fn test_new_uninitialized_with_non_power_of_two_buckets() {
    use core::alloc::Layout;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let buckets = 3; // Not a power of two
    let fallibility = Fallibility::Fallible;

    let result = unsafe { RawTable::<usize, TestAllocator>::new_uninitialized(allocator, buckets, fallibility) };

    assert!(result.is_err());
}

#[test]
fn test_new_uninitialized_with_zero_buckets() {
    use core::alloc::Layout;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let buckets = 0; // Zero buckets, should trigger an error
    let fallibility = Fallibility::Fallible;

    let result = unsafe { RawTable::<usize, TestAllocator>::new_uninitialized(allocator, buckets, fallibility) };

    assert!(result.is_err());
}

