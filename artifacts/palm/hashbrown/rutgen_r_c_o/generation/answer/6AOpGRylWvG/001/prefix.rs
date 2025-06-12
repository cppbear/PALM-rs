// Answer 0

#[test]
#[should_panic]
fn test_with_capacity_exceeding_max() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
            Err(AllocError)
        }
    }

    let alloc = DummyAllocator;
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    
    let capacity = usize::MAX + 1;
    with_capacity(&alloc, table_layout, capacity);
}

#[test]
#[should_panic]
fn test_with_capacity_exceeding_max_plus_ten() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, AllocError> {
            Err(AllocError)
        }
    }

    let alloc = DummyAllocator;
    let table_layout = TableLayout { size: 16, ctrl_align: 8 };
    
    let capacity = usize::MAX + 10;
    with_capacity(&alloc, table_layout, capacity);
}

