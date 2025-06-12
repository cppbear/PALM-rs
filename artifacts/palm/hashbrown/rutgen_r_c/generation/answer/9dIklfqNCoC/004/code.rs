// Answer 0

#[test]
fn test_find_inner_found() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary methods for allocator
    }

    let alloc = TestAllocator;

    let table_layout = TableLayout::default();
  
    // Create a RawTableInner with predetermined capacity
    let mut table = unsafe {
        RawTableInner::fallible_with_capacity(&alloc, table_layout, 8, Fallibility::Infallible)
            .expect("Initialization failed")
    };

    // Simulate a state where we have FULL buckets and a known hash
    let hash = 42;
    let index_found = 2; // This is the index we want to find
    let tag = Tag::full(hash);

    // Simulate that all buckets are full and their tags are matching
    unsafe {
        table.set_ctrl(index_found, tag);
        // We set other full buckets to a different tag to ensure only the target matches
        for i in 0..8 {
            if i != index_found {
                table.set_ctrl(i, Tag::full(i as u64));
            }
        }
    }

    // Create an `eq` function that only returns true for our expected index
    let mut eq = |idx: usize| idx == index_found;

    // Call the find_inner method
    let result = unsafe { table.find_inner(hash, &mut eq) };

    assert_eq!(result, Some(index_found));
}

#[test]
fn test_find_inner_not_found() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement necessary methods for allocator
    }

    let alloc = TestAllocator;

    let table_layout = TableLayout::default();

    // Create a RawTableInner with predetermined capacity
    let mut table = unsafe {
        RawTableInner::fallible_with_capacity(&alloc, table_layout, 8, Fallibility::Infallible)
            .expect("Initialization failed")
    };

    let hash = 42;
    let tag = Tag::full(hash);

    // Simulate that all buckets are full with different tags
    unsafe {
        for i in 0..8 {
            table.set_ctrl(i, Tag::full(i as u64));
        }
    }

    // Create an `eq` function that never returns true
    let mut eq = |_: usize| false;

    // Call the find_inner method
    let result = unsafe { table.find_inner(hash, &mut eq) };

    assert_eq!(result, None);
}

