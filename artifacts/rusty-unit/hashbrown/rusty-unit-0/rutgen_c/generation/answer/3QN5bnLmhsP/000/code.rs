// Answer 0

#[test]
fn test_fmt_debug_for_drain() {
    struct MockAllocator;
    
    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    struct MockRawIter;

    impl MockRawIter {
        pub fn clone(&self) -> Self {
            MockRawIter
        }
    }

    struct MockRawTableInner;

    struct MockRawTable;

    struct MockIter {
        items: usize,
    }

    impl MockIter {
        pub fn new(items: usize) -> Self {
            MockIter { items }
        }
    }

    impl RawIter<usize> {
        pub fn iter(&self) -> MockRawIter {
            MockRawIter
        }
    }

    let inner = RawDrain {
        iter: RawIter {
            iter: MockRawTable,
            items: 0,
        },
        table: MockRawTableInner,
        orig_table: NonNull::dangling(),
        marker: PhantomData,
    };

    let drain = Drain {
        inner,
    };

    let mut buffer = Vec::new();
    let result = drain.fmt(&mut buffer);
    assert!(result.is_ok());
}

