// Answer 0

#[test]
fn test_raw_drain_iter() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let dummy_alloc = DummyAllocator;

    let raw_table_inner = RawTableInner {
        bucket_mask: 0b11111111,
        ctrl: NonNull::dangling(),
        growth_left: 1,
        items: 5,
    };

    let raw_table = RawTable {
        table: raw_table_inner,
        alloc: dummy_alloc,
        marker: PhantomData,
    };

    let raw_drain = RawDrain {
        iter: RawIter {
            iter: RawIterRange { /* initialize with appropriate values */ },
            items: 5,
        },
        table: raw_table_inner,
        orig_table: NonNull::from(&raw_table_inner),
        marker: PhantomData,
    };

    let result_iter = raw_drain.iter();
    assert_eq!(result_iter.items, 5);
    // Here we can add more checks on the structure of RawIter if we define RawIterRange appropriately.
}

#[test]
fn test_raw_drain_iter_empty() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let dummy_alloc = DummyAllocator;

    let raw_table_inner = RawTableInner {
        bucket_mask: 0,
        ctrl: NonNull::dangling(),
        growth_left: 0,
        items: 0,
    };

    let raw_table = RawTable {
        table: raw_table_inner,
        alloc: dummy_alloc,
        marker: PhantomData,
    };

    let raw_drain = RawDrain {
        iter: RawIter {
            iter: RawIterRange { /* initialize as empty */ },
            items: 0,
        },
        table: raw_table_inner,
        orig_table: NonNull::from(&raw_table_inner),
        marker: PhantomData,
    };

    let result_iter = raw_drain.iter();
    assert_eq!(result_iter.items, 0);
}

