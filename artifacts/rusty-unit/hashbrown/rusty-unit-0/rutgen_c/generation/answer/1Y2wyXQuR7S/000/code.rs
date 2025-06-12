// Answer 0

#[test]
fn test_bucket_ptr_valid() {
    use std::alloc::Allocator;
    use std::alloc::Global;

    struct TableLayout;

    impl TableLayout {
        fn calculate_layout_for(&self, _buckets: usize) -> Option<(Layout, usize)> {
            let layout = Layout::array::<u8>(4).ok()?;
            Some((layout, 0))
        }
    }

    struct Fallibility;

    impl Fallibility {
        fn capacity_overflow(&self) -> TryReserveError {
            // Stub implementation
            TryReserveError::CapacityOverflow
        }

        fn alloc_err(&self, _layout: Layout) -> TryReserveError {
            // Stub implementation
            TryReserveError::AllocError
        }
    }

    let alloc = Global;
    let layout = TableLayout;
    let mut raw_table = RawTableInner::with_capacity(&alloc, layout, 4);

    // Calling bucket_ptr with index 0 and size_of equal to 1 (size of u8)
    let pointer = unsafe { raw_table.bucket_ptr(0, std::mem::size_of::<u8>()) };
    assert!(!pointer.is_null());
}

#[test]
#[should_panic]
fn test_bucket_ptr_out_of_bounds() {
    use std::alloc::Allocator;
    use std::alloc::Global;

    struct TableLayout;

    impl TableLayout {
        fn calculate_layout_for(&self, _buckets: usize) -> Option<(Layout, usize)> {
            let layout = Layout::array::<u8>(4).ok()?;
            Some((layout, 0))
        }
    }

    struct Fallibility;

    impl Fallibility {
        fn capacity_overflow(&self) -> TryReserveError {
            TryReserveError::CapacityOverflow
        }

        fn alloc_err(&self, _layout: Layout) -> TryReserveError {
            TryReserveError::AllocError
        }
    }

    let alloc = Global;
    let layout = TableLayout;
    let mut raw_table = RawTableInner::with_capacity(&alloc, layout, 4);

    // Calling bucket_ptr with an out of bounds index
    unsafe { raw_table.bucket_ptr(5, std::mem::size_of::<u8>()) };
}

#[test]
#[should_panic]
fn test_bucket_ptr_zero_size() {
    use std::alloc::Allocator;
    use std::alloc::Global;

    struct TableLayout;

    impl TableLayout {
        fn calculate_layout_for(&self, _buckets: usize) -> Option<(Layout, usize)> {
            let layout = Layout::array::<u8>(4).ok()?;
            Some((layout, 0))
        }
    }

    struct Fallibility;

    impl Fallibility {
        fn capacity_overflow(&self) -> TryReserveError {
            TryReserveError::CapacityOverflow
        }

        fn alloc_err(&self, _layout: Layout) -> TryReserveError {
            TryReserveError::AllocError
        }
    }

    let alloc = Global;
    let layout = TableLayout;
    let mut raw_table = RawTableInner::with_capacity(&alloc, layout, 4);

    // Calling bucket_ptr with size_of equal to 0 (which is invalid)
    unsafe { raw_table.bucket_ptr(0, 0) };
}

