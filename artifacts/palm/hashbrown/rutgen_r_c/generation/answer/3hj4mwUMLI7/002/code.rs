// Answer 0

#[test]
fn test_allocation_size_or_zero_non_empty() {
    use crate::alloc::Global;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required methods for allocator here if needed
    }

    unsafe fn initialize_table() -> RawTableInner {
        let alloc = TestAllocator;
        let layout = TableLayout { size: 64, ctrl_align: 8 };
        let capacity = 8; // Non-zero capacity
        
        RawTableInner::with_capacity(&alloc, layout, capacity)
    }

    unsafe {
        let table = initialize_table();
        let layout = TableLayout { size: 64, ctrl_align: 8 };
        let allocation_size = table.allocation_size_or_zero(layout);
        
        assert!(allocation_size > 0); // Expecting a non-zero allocation size for non-empty table
    }
}

#[test]
#[should_panic]
fn test_allocation_size_or_zero_empty() {
    use crate::alloc::Global;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        // Implement required methods for allocator here if needed
    }

    unsafe fn create_empty_table() -> RawTableInner {
        let alloc = TestAllocator;
        let layout = TableLayout { size: 64, ctrl_align: 8 };
        let capacity = 0; // Zero capacity for empty table
        
        RawTableInner::with_capacity(&alloc, layout, capacity)
    }

    unsafe {
        let table = create_empty_table();
        let layout = TableLayout { size: 64, ctrl_align: 8 };
        let allocation_size = table.allocation_size_or_zero(layout);

        assert_eq!(allocation_size, 0); // Should return 0 for empty table
    }
}

