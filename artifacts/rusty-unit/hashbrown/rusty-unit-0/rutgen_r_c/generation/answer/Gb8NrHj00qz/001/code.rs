// Answer 0

#[test]
fn test_iter_empty_drain() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let drain: Drain<u32, u32, TestAllocator> = Drain {
        inner: RawDrain {
            iter: RawIter {
                iter: RawIterRange { start: 0, end: 0 },
                items: 0,
            },
            table: RawTableInner,
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };

    let iter = drain.iter();
    assert_eq!(iter.inner.items, 0);
}

#[test]
fn test_iter_single_element_drain() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let drain: Drain<u32, u32, TestAllocator> = Drain {
        inner: RawDrain {
            iter: RawIter {
                iter: RawIterRange { start: 0, end: 1 },
                items: 1,
            },
            table: RawTableInner,
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };

    let iter = drain.iter();
    assert_eq!(iter.inner.items, 1);
}

#[test]
fn test_iter_multiple_elements_drain() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let drain: Drain<u32, u32, TestAllocator> = Drain {
        inner: RawDrain {
            iter: RawIter {
                iter: RawIterRange { start: 0, end: 10 },
                items: 10,
            },
            table: RawTableInner,
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };

    let iter = drain.iter();
    assert_eq!(iter.inner.items, 10);
}

