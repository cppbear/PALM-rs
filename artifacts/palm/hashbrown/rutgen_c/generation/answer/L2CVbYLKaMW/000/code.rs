// Answer 0

#[test]
fn test_new_uninitialized_with_power_of_two_buckets() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = TestAllocator;
    
    let result = RawTable::<i32, TestAllocator>::new_uninitialized(allocator, 4, Fallibility::Infallible);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_new_uninitialized_with_non_power_of_two_buckets() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = TestAllocator;
    
    // This should panic due to the assertion in new_uninitialized
    let _ = unsafe { RawTable::<i32, TestAllocator>::new_uninitialized(allocator, 3, Fallibility::Infallible) };
}

#[test]
fn test_new_uninitialized_with_capacity_zero() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let allocator = TestAllocator;
    
    let result = RawTable::<i32, TestAllocator>::new_uninitialized(allocator, 0, Fallibility::Infallible);
    
    // Expecting Ok with no allocations since the capacity is zero
    assert!(result.is_ok());
}

