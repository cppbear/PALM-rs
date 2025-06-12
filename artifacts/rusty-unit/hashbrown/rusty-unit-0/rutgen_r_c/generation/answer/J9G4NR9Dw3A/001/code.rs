// Answer 0

#[test]
fn test_prepare_resize_success() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement required methods for the Allocator trait
    }

    let allocator = MockAllocator;
    let table_layout = TableLayout { size: 8, ctrl_align: 8 };

    let table = RawTableInner::with_capacity(&allocator, table_layout, 8);
    let fallibility = Fallibility::Fallible;

    // Should succeed, as the capacity is equal to items
    let result = table.prepare_resize(&allocator, table_layout, 8, fallibility);
    assert!(result.is_ok());
}

#[test]
fn test_prepare_resize_fail_capacity_overflow() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Implement required methods for the Allocator trait
    }

    let allocator = MockAllocator;
    let table_layout = TableLayout { size: 8, ctrl_align: 8 };

    let table = RawTableInner::with_capacity(&allocator, table_layout, usize::MAX);
    let fallibility = Fallibility::Fallible;

    // Here we expect a capacity overflow error
    let result = table.prepare_resize(&allocator, table_layout, usize::MAX, fallibility);
    assert!(result.is_err());
    if let Err(TryReserveError::CapacityOverflow) = result {
        // The error is expected
    } else {
        panic!("Expected CapacityOverflow error");
    }
}

#[test]
fn test_prepare_resize_fail_alloc_error() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        // Simulate allocation failure for testing
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, AllocError> {
            Err(AllocError) // Trigger error
        }
    }

    let allocator = MockAllocator;
    let table_layout = TableLayout { size: 8, ctrl_align: 8 };

    let table = RawTableInner::with_capacity(&allocator, table_layout, 8);
    let fallibility = Fallibility::Fallible;

    // Should fail due to the allocation error
    let result = table.prepare_resize(&allocator, table_layout, 8, fallibility);
    assert!(result.is_err());
    if let Err(TryReserveError::AllocError { layout: _ }) = result {
        // The error is expected
    } else {
        panic!("Expected AllocError error");
    }
}

