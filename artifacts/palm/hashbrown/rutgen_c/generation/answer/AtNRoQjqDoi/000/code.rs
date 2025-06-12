// Answer 0

#[test]
fn test_resize_increases_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(64, 8).unwrap())))
        }
        
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), Layout::from_size_align(64, 8).unwrap());
        }
    }

    let mut table = RawTable::new_in(TestAllocator);
    let initial_capacity = 4;
    table = RawTable::with_capacity_in(initial_capacity, TestAllocator);
    let items_before = table.len();
    let new_capacity = 8;

    unsafe {
        let result = table.resize(new_capacity, |&x| x as u64, Fallibility::Infallible);
        assert!(result.is_ok());
        assert!(table.capacity() >= new_capacity);
        assert_eq!(table.len(), items_before);
    }
}

#[test]
#[should_panic]
fn test_resize_zero_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(64, 8).unwrap())))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), Layout::from_size_align(64, 8).unwrap());
        }
    }

    let mut table = RawTable::new_in(TestAllocator);
    table = RawTable::with_capacity_in(4, TestAllocator);
    
    unsafe {
        let _ = table.resize(0, |&x| x as u64, Fallibility::Infallible); // Should panic
    }
}

