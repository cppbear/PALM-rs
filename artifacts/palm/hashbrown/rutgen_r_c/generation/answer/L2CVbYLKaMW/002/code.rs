// Answer 0

#[test]
fn test_new_uninitialized_with_power_of_two_buckets() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(0x1 as *mut u8).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = DummyAllocator;
    let buckets = 4; // 4 is a power of two
    let fallibility = Fallibility::Infallible;

    let result: Result<RawTable<usize, DummyAllocator>, TryReserveError> = 
        unsafe { RawTable::<usize, DummyAllocator>::new_uninitialized(allocator, buckets, fallibility) };

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_new_uninitialized_with_non_power_of_two_buckets() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(0x1 as *mut u8).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = DummyAllocator;
    let buckets = 3; // 3 is not a power of two
    let fallibility = Fallibility::Infallible;

    // This should panic due to the buckets not being a power of two
    let _ = unsafe { RawTable::<usize, DummyAllocator>::new_uninitialized(allocator, buckets, fallibility) };
}

#[test]
fn test_new_uninitialized_with_capacity_overflow() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = DummyAllocator;
    let buckets = 4; // 4 is a power of two
    let fallibility = Fallibility::Infallible;

    let result: Result<RawTable<usize, DummyAllocator>, TryReserveError> = 
        unsafe { RawTable::<usize, DummyAllocator>::new_uninitialized(allocator, buckets, fallibility) };

    assert!(result.is_err());
    if let Err(TryReserveError::CapacityOverflow) = result {
        // Test passed as we expected a capacity overflow.
    } else {
        panic!("Expected capacity overflow error");
    }
}

