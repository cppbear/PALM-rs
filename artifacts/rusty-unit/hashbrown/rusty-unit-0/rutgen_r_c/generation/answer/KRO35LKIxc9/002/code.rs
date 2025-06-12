// Answer 0

#[test]
fn test_ctrl_index_out_of_bounds() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, TryReserveError> {
            let size = layout.size();
            let align = layout.align();
            let ptr = unsafe { alloc::alloc::alloc(layout) };
            NonNull::new(ptr).ok_or(TryReserveError::AllocError)
        }
        
        fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            // Stub
        }
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assume default is appropriate.
    let capacity = 2; // Arbitrary capacity for simplicity.

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    let index = raw_table.num_ctrl_bytes(); // This should trigger the panic.

    unsafe {
        // The below call should panic as we are using an out-of-bounds index.
        let _ = raw_table.ctrl(index);
    }
}

#[test]
#[should_panic]
fn test_ctrl_with_high_index() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, TryReserveError> {
            let size = layout.size();
            let align = layout.align();
            let ptr = unsafe { alloc::alloc::alloc(layout) };
            NonNull::new(ptr).ok_or(TryReserveError::AllocError)
        }
        
        fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            // Stub
        }
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout::default(); // Assume default is appropriate.
    let capacity = 2; // Arbitrary capacity for simplicity.

    let mut raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    let index = raw_table.num_ctrl_bytes() + 1; // Out-of-bounds index.

    unsafe {
        // The below call is expected to panic due to an out-of-bounds index.
        let _ = raw_table.ctrl(index);
    }
}

