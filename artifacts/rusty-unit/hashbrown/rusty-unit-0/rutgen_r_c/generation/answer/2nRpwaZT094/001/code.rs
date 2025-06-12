// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 1]))))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            let _ = Box::from_raw(ptr.as_ptr() as *mut [u8; 1]); // Deallocate correctly
        }
    }

    let alloc = TestAllocator;
    let table = RawTable::<i32, TestAllocator>::with_capacity_in(0, alloc);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 1]))))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            let _ = Box::from_raw(ptr.as_ptr() as *mut [u8; 1]); // Deallocate correctly
        }
    }

    let alloc = TestAllocator;
    let table = RawTable::<i32, TestAllocator>::with_capacity_in(1, alloc);
    assert!(table.capacity() >= 1);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 1]))))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            let _ = Box::from_raw(ptr.as_ptr() as *mut [u8; 1]); // Deallocate correctly
        }
    }

    let alloc = TestAllocator;
    let table = RawTable::<i32, TestAllocator>::with_capacity_in(16, alloc);
    assert!(table.capacity() >= 16);
}

#[should_panic] 
#[test]
fn test_with_capacity_in_exceeding_size() {
    struct PanicAllocator;

    unsafe impl Allocator for PanicAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = PanicAllocator;
    let _table = RawTable::<i32, PanicAllocator>::with_capacity_in(usize::MAX, alloc);
}

