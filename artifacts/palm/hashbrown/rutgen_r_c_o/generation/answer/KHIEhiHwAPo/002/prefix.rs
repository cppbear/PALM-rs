// Answer 0

#[test]
fn test_new_uninitialized_valid_cases() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let size = layout.size();
            let align = layout.align();
            let ptr = alloc::alloc::alloc(layout);
            if !ptr.is_null() {
                Ok(NonNull::new(ptr).unwrap())
            } else {
                Err(())
            }
        }
    }

    let alloc = DummyAllocator;
    let table_layout = TableLayout::new::<u8>();

    unsafe {
        let result_1 = RawTableInner::new_uninitialized(&alloc, table_layout, 1, Fallibility::Infallible);
        let result_2 = RawTableInner::new_uninitialized(&alloc, table_layout, 2, Fallibility::Infallible);
        let result_4 = RawTableInner::new_uninitialized(&alloc, table_layout, 4, Fallibility::Infallible);
        let result_8 = RawTableInner::new_uninitialized(&alloc, table_layout, 8, Fallibility::Infallible);
        let result_16 = RawTableInner::new_uninitialized(&alloc, table_layout, 16, Fallibility::Infallible);
        let result_32 = RawTableInner::new_uninitialized(&alloc, table_layout, 32, Fallibility::Infallible);
        let result_64 = RawTableInner::new_uninitialized(&alloc, table_layout, 64, Fallibility::Infallible);
        let result_128 = RawTableInner::new_uninitialized(&alloc, table_layout, 128, Fallibility::Infallible);
        let result_256 = RawTableInner::new_uninitialized(&alloc, table_layout, 256, Fallibility::Infallible);
    }
}

#[test]
#[should_panic]
fn test_new_uninitialized_invalid_buckets() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
    }
    
    let alloc = DummyAllocator;
    let table_layout = TableLayout::new::<u8>();

    unsafe {
        let _ = RawTableInner::new_uninitialized(&alloc, table_layout, 3, Fallibility::Infallible);
    }
}

#[test]
fn test_new_uninitialized_large_valid_cases() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
    }

    let alloc = DummyAllocator;
    let table_layout = TableLayout::new::<u8>();

    unsafe {
        let result_1024 = RawTableInner::new_uninitialized(&alloc, table_layout, 1024, Fallibility::Infallible);
        let result_2048 = RawTableInner::new_uninitialized(&alloc, table_layout, 2048, Fallibility::Infallible);
        let result_4096 = RawTableInner::new_uninitialized(&alloc, table_layout, 4096, Fallibility::Infallible);
        let result_8192 = RawTableInner::new_uninitialized(&alloc, table_layout, 8192, Fallibility::Infallible);
    }
}

