// Answer 0

#[test]
fn test_find_inner_with_full_and_empty_buckets() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement the required methods for the allocator...
    }

    // Create instance for testing
    let alloc = MockAllocator;
    let table_layout = TableLayout::default();
    let capacity = 8; // Set the initial capacity (8, power of two)
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    // Fill in the control bytes as 'FULL' for test purposes.
    for i in 0..capacity {
        unsafe {
            raw_table.set_ctrl_hash(i, i as u64); // Set hash for each bucket
            raw_table.set_ctrl(i, Tag::full(i as u64)); // Setting tag as 'FULL'
        }
    }

    // Create eq function that matches the indices we have set
    let mut eq = |index: usize| index % 2 == 0; // This function will return true for even indices

    // Hash of the tag we are searching for
    let tag_hash = Tag::full(2 as u64);

    // Call the unsafe function under test
    let result = unsafe { raw_table.find_inner(tag_hash.0.into(), &mut eq) };

    assert_eq!(result, Some(2)); // We expect to find the index 2 as it matches the eq function
}

#[test]
fn test_find_inner_with_no_matches() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement the required methods for the allocator...
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::default();
    let capacity = 8; // Power of two
    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    // Fill control bytes
    for i in 0..capacity {
        unsafe {
            raw_table.set_ctrl_hash(i, i as u64); // Set hash for each bucket
            raw_table.set_ctrl(i, Tag::full(i as u64)); // Set all tags to 'FULL'
        }
    }

    // Create eq function that will not find any matches
    let mut eq = |index: usize| index > capacity; // This function will always return false

    let tag_hash = Tag::full(2 as u64);
    
    // Call the unsafe function under test
    let result = unsafe { raw_table.find_inner(tag_hash.0.into(), &mut eq) };

    assert_eq!(result, None); // We expect to find no matches
}

