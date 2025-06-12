// Answer 0

#[test]
fn test_iter_basic() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required allocator methods for testing
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assuming a default value exists
    let capacity = 4; // Power of two for bucket count
    let raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    unsafe {
        let iter = raw_table_inner.iter::<u32>();
        assert_eq!(iter.items, raw_table_inner.items);
    }
}

#[test]
fn test_iter_empty() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required allocator methods for testing
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assuming a default value exists
    let capacity = 0; // Empty capacity
    let raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    unsafe {
        let iter = raw_table_inner.iter::<u32>();
        assert_eq!(iter.items, raw_table_inner.items); // Should be zero
    }
}

#[test]
fn test_iter_full() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required allocator methods for testing
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assuming a default value exists
    let capacity = 8; // Power of two
    let mut raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Simulating adding items to the RawTableInner
    raw_table_inner.items = 8; // Assuming we mark 8 items

    unsafe {
        let iter = raw_table_inner.iter::<u32>();
        assert_eq!(iter.items, raw_table_inner.items); // Should be equal to number of items
    }
}

#[should_panic]
#[test]
fn test_iter_invalid_control() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required allocator methods for testing
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assuming a default value exists
    let capacity = 4; // Power of two for bucket count
    let mut raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);

    // Simulating a condition where control bytes are not properly initialized
    // This simulates a panic scenario
    raw_table_inner.ctrl = NonNull::dangling(); // Invalid control bytes

    unsafe {
        let _iter = raw_table_inner.iter::<u32>(); // Should panic
    }
}

#[test]
fn test_iter_non_zero_length() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required allocator methods for testing
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assuming a default value exists
    let capacity = 16; // Power of two for bucket count
    let raw_table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    // Marking some items as present
    unsafe {
        raw_table_inner.items = 10; // Assuming 10 active items
        let iter = raw_table_inner.iter::<u32>();
        assert_eq!(iter.items, raw_table_inner.items); // Should match the active item count
    }
}

