// Answer 0

#[test]
fn test_insert_with_growth_left() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for testing
        }
    }

    let allocator = TestAllocator;

    let mut table: RawTable<u8, TestAllocator> = RawTable::with_capacity_in(4, allocator);
    
    // Simulate a state where growth_left is 0 and ctrl shows that we have a tombstone
    table.table.growth_left = 0;

    // Set up a control byte indicating a tombstone
    let index = 0;
    let tombstone = Tag(0b1000_0000); // DELETED
    unsafe {
        *table.table.ctrl(index) = tombstone;
    }

    // This should call reserve and insert successfully
    let bucket = table.insert(1, 42, |&x| x as u64);
    assert!(bucket.ptr.as_ptr().is_non_null());
}

#[test]
fn test_insert_with_empty_tombstone() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for testing
        }
    }

    let allocator = TestAllocator;

    let mut table: RawTable<u8, TestAllocator> = RawTable::with_capacity_in(4, allocator);
    
    // Simulate a state where growth_left is 0 and ctrl shows that we have a tombstone
    table.table.growth_left = 0;

    // Set up a control byte indicating an empty tombstone
    let index = 1; // Assume non-zero index
    let empty_tombstone = Tag(0b1111_1111); // EMPTY
    unsafe {
        *table.table.ctrl(index) = empty_tombstone;
    }

    // This should call reserve and insert successfully since old_ctrl is special (EMPTY)
    let bucket = table.insert(2, 99, |&x| x as u64);
    assert!(bucket.ptr.as_ptr().is_non_null());
}

