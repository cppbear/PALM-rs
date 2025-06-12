// Answer 0

#[test]
fn test_len_empty_table() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table: RawTable<i32, TestAllocator> = RawTable::new_in(alloc);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_len_with_elements() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::with_capacity_in(4, alloc);
    
    // Here we assume insert method exists for adding elements to the table
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
    }
    
    assert_eq!(table.len(), 2);
}

#[test]
fn test_len_after_clear() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::with_capacity_in(4, alloc);
    
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
        table.clear();
    }
    
    assert_eq!(table.len(), 0);
}

