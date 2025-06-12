// Answer 0

#[test]
fn test_new_uninitialized_valid() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simulate successful allocation for this test
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let fallibility = Fallibility::Infallible;

    // Test with valid power of two buckets
    let result = unsafe { RawTable::<i32, TestAllocator>::new_uninitialized(allocator, 4, fallibility) };
    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_new_uninitialized_buckets_not_power_of_two() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let fallibility = Fallibility::Infallible;

    // Test with buckets that are not a power of two (e.g., 3)
    let _ = unsafe { RawTable::<i32, TestAllocator>::new_uninitialized(allocator, 3, fallibility) };
}

#[test]
fn test_new_uninitialized_alloc_error() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            // Simulate allocation failure
            Err(())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let fallibility = Fallibility::Fallible;

    // Test for allocation error with valid power of two buckets
    let result = unsafe { RawTable::<i32, TestAllocator>::new_uninitialized(allocator, 4, fallibility) };
    assert!(result.is_err());
}

