// Answer 0

#[test]
fn test_shrink_to_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    table.shrink_to(0, |x| *x as u64);
    assert!(table.len() == 0);
}

#[test]
fn test_shrink_to_non_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::with_capacity_in(16, allocator);
    // Assume items have been inserted here to set table.items > 0
    unsafe { table.table.items = 10; }
    
    table.shrink_to(5, |x| *x as u64);
    assert!(table.len() >= 5);
}

#[test]
fn test_shrink_to_equal() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::with_capacity_in(8, allocator);
    
    unsafe { table.table.items = 6; }
    table.shrink_to(6, |x| *x as u64);
    assert!(table.len() == 6);
}

