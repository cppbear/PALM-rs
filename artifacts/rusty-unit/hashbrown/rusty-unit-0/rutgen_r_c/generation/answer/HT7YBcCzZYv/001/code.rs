// Answer 0

#[test]
fn test_find_or_find_insert_slot_returns_err() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling()) // Simulated allocation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;

    let mut raw_table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(4, allocator);

    // Setup a condition where the table is full, triggering the Err condition
    let hash: u64 = 12345;
    let eq = |item: &u64| *item == 50; // Value we won't find, simulating a non-existing item

    // Simulate insertions to fill the table
    for i in 0..4 {
        raw_table.insert(i as u64, i as u64, |x| *x);
    }

    // This should trigger the Err condition since all buckets are full
    let result = raw_table.find_or_find_insert_slot(hash, eq, |x| *x);
    
    assert!(result.is_err());
}

#[test]
fn test_find_or_find_insert_slot_non_existing_item() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling()) // Simulated allocation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;

    let mut raw_table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(4, allocator);

    // Setup a condition where the table allows space for new elements and has existing ones
    let eq = |item: &u64| *item == 100; // Value we won't find

    // Fill some values to the table, ensuring there are empty slots
    raw_table.insert(1, 1, |x| *x);
    raw_table.insert(2, 2, |x| *x);
    
    // Explicitly set hash to not match any items in the table
    let hash: u64 = 10000; 

    // Since our table should have space, this should return an Err with a slot to insert.
    let result = raw_table.find_or_find_insert_slot(hash, eq, |x| *x);
    
    assert!(result.is_err());
}

