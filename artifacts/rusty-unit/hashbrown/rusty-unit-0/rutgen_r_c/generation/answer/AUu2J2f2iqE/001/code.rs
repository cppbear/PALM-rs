// Answer 0

#[test]
fn test_iter_hash_valid() {
    use core::alloc::Global;

    // Create a new RawTable with a specific capacity
    let alloc = Global;
    let mut table = RawTable::<u32, Global>::with_capacity_in(8, alloc);

    // Insert values into the RawTable that we can later match against the hash
    let hash_value = 42;
    unsafe {
        table.insert(hash_value, 10, |value| *value)
            .unwrap();
        table.insert(hash_value, 20, |value| *value)
            .unwrap();
    }

    // Create the iterator for the specified hash
    let iter = unsafe { table.iter_hash(hash_value) };

    // Validate the values returned by the iterator
    let values: Vec<_> = iter.collect();
    assert_eq!(values, vec![10, 20]);
}

#[test]
#[should_panic]
fn test_iter_hash_outlive() {
    struct DummyAllocator;

    // Implement a dummy allocator since we don't need actual allocation for this test 
    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    // Create a RawTable with the dummy allocator
    let alloc = DummyAllocator;
    let table: RawTable<u32, DummyAllocator> = RawTable::new_in(alloc);

    // Create an iterator without actual items (since we didn't insert any)
    let iter = unsafe { table.iter_hash(10) }; // The hash used here does not have any items

    // Attempt to fetch values from the iterator; expected to panic
    let _ = iter.next();
}

#[test]
fn test_iter_hash_non_matching_hash() {
    use core::alloc::Global;

    let alloc = Global;
    let mut table = RawTable::<u32, Global>::with_capacity_in(8, alloc);

    // Insert values with different hashes (but we will only test one specific hash)
    unsafe {
        table.insert(1, 10, |value| *value);
        table.insert(2, 20, |value| *value);
    }

    // Create an iterator for a hash that has no matching items
    let iter = unsafe { table.iter_hash(42) }; // This hash should yield no results

    // Validate that no items are returned
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_hash_empty_table() {
    use core::alloc::Global;

    let alloc = Global;
    let table = RawTable::<u32, Global>::with_capacity_in(8, alloc);

    // Create an iterator on an empty table
    let iter = unsafe { table.iter_hash(42) }; // The hash value doesn't matter here

    // Validate that the iterator returns no items
    assert!(iter.next().is_none());
}

