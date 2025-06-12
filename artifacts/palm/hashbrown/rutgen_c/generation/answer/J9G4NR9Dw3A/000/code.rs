// Answer 0

#[test]
fn test_prepare_resize_success() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        // Implement necessary methods based on the allocation requirements
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout { size: 64, ctrl_align: 8 };
    let initial_capacity = 8;

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, initial_capacity);
    raw_table.items = 4; // Set some items

    let result = raw_table.prepare_resize(&alloc, table_layout, 16, Fallibility::Infallible);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_prepare_resize_capacity_overflow() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        // Implement necessary methods for allocation
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout { size: 64, ctrl_align: 8 };
    let initial_capacity = 8;

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, initial_capacity);
    raw_table.items = 4; // Set some items

    // Try resizing with too large capacity
    let _ = raw_table.prepare_resize(&alloc, table_layout, usize::MAX, Fallibility::Infallible);
}

#[test]
fn test_prepare_resize_check_table_layout() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        // Implement necessary methods for allocation
    }

    let alloc = TestAllocator;
    let valid_table_layout = TableLayout { size: 64, ctrl_align: 8 };
    let invalid_table_layout = TableLayout { size: 128, ctrl_align: 16 };
    let initial_capacity = 8;

    let mut raw_table = RawTableInner::with_capacity(&alloc, valid_table_layout, initial_capacity);
    raw_table.items = 4; // Set some items

    // Should succeed
    let result = raw_table.prepare_resize(&alloc, valid_table_layout, 16, Fallibility::Infallible);
    assert!(result.is_ok());

    // Should panic due to mismatched table layout
    let _ = raw_table.prepare_resize(&alloc, invalid_table_layout, 16, Fallibility::Infallible);
}

