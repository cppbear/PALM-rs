// Answer 0

#[test]
fn test_shrink_to_empty_table() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    // Initial capacity of the table to ensure it has some buckets
    let initial_capacity = 16;
    table.table = RawTableInner::with_capacity(&table.alloc, RawTable::TABLE_LAYOUT, initial_capacity);
    
    // Ensuring the table has some items to begin with
    for i in 0..8 {
        let _ = table.insert(i as u64, i, |x| *x); // Inserting values
    }

    table.shrink_to(0, |x| *x);
    assert_eq!(table.len(), 0);
    assert_eq!(table.buckets(), initial_capacity);
}

#[test]
fn test_shrink_to_boundary_case() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    let initial_capacity = 8;
    table.table = RawTableInner::with_capacity(&table.alloc, RawTable::TABLE_LAYOUT, initial_capacity);

    for i in 0..4 {
        let _ = table.insert(i as u64, i, |x| *x); // Inserting values
    }

    // The min_size matches the current number of items and ensures capacity is adequate
    table.shrink_to(4, |x| *x);
    assert_eq!(table.len(), 4);
    assert_eq!(table.buckets(), initial_capacity);
}

#[test]
fn test_shrink_to_more_than_needed() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    let initial_capacity = 16;
    table.table = RawTableInner::with_capacity(&table.alloc, RawTable::TABLE_LAYOUT, initial_capacity);

    for i in 0..8 {
        let _ = table.insert(i as u64, i, |x| *x); // Inserting values
    }

    // The min_size is less than current length, ensuring we do not shrink unnecessarily
    table.shrink_to(7, |x| *x);
    assert_eq!(table.len(), 8);
    assert_eq!(table.buckets(), initial_capacity);
}

