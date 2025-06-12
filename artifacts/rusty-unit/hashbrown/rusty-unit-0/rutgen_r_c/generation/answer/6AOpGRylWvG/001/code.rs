// Answer 0

#[test]
#[should_panic]
fn test_with_capacity_alloc_error() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
            Err(AllocErr)
        }
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout {
        size: 8,
        ctrl_align: 4,
    };
    let capacity = 10;
    
    // This should trigger a panic due to allocation failure.
    let _table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
}

#[test]
#[should_panic]
fn test_with_capacity_capacity_overflow() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
            unreachable!()
        }
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout {
        size: 8,
        ctrl_align: 4,
    };
    let capacity = isize::MAX as usize + 1;

    // This should trigger a panic due to capacity overflow.
    let _table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
}

