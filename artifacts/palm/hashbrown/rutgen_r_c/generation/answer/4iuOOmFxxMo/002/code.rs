// Answer 0

#[test]
fn test_resize_inner_success() {
    use crate::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement necessary methods...
    }

    let allocator = DummyAllocator;
    let layout = TableLayout { size: 8, ctrl_align: 1 };

    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    // Mimic some initial state
    raw_table.items = 2;

    unsafe {
        let result = raw_table.resize_inner(
            &allocator,
            4,
            &|_, index| index as u64, // Simple hashing function
            Fallibility::Infallible,
            layout,
        );

        assert!(result.is_ok());
    }
}

#[test]
fn test_resize_inner_invalid_capacity() {
    use crate::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement necessary methods...
    }

    let allocator = DummyAllocator;
    let layout = TableLayout { size: 8, ctrl_align: 1 };

    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    // Mimic some initial state
    raw_table.items = 2;

    unsafe {
        let result = raw_table.resize_inner(
            &allocator,
            2,
            &|_, index| index as u64, // Simple hashing function
            Fallibility::Infallible,
            layout,
        );

        // Since the capacity is less than items, it should return an error.
        assert!(result.is_err());
    }
}

#[test]
fn test_resize_inner_empty_self() {
    use crate::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement necessary methods...
    }

    let allocator = DummyAllocator;
    let layout = TableLayout { size: 8, ctrl_align: 1 };

    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 0);
    raw_table.items = 0; // No items

    unsafe {
        let result = raw_table.resize_inner(
            &allocator,
            1,
            &|_, index| index as u64, // Simple hashing function
            Fallibility::Infallible,
            layout,
        );

        assert!(result.is_ok());
    }
} 

#[test]
#[should_panic]
fn test_resize_inner_panic_empty_capacity() {
    use crate::alloc::Global;

    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        // Implement necessary methods...
    }

    let allocator = DummyAllocator;
    let layout = TableLayout { size: 8, ctrl_align: 1 };

    let mut raw_table = RawTableInner::with_capacity(&allocator, layout, 4);
    raw_table.items = 2;

    unsafe {
        // This should panic due to the self.items != 0 and capacity == 0 constraints.
        let _ = raw_table.resize_inner(
            &allocator,
            0,
            &|_, index| index as u64, // Simple hashing function
            Fallibility::Infallible,
            layout,
        );
    }
}

