// Answer 0

#[test]
fn test_clone_from_spec() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut source = RawTable {
        table: RawTableInner {
            bucket_mask: 0,
            ctrl: NonNull::new_unchecked(0 as *mut u8),
            growth_left: 0,
            items: 0,
        },
        alloc: TestAllocator,
        marker: PhantomData,
    };

    let mut target = RawTable {
        table: RawTableInner {
            bucket_mask: 0,
            ctrl: NonNull::new_unchecked(0 as *mut u8),
            growth_left: 0,
            items: 0,
        },
        alloc: TestAllocator,
        marker: PhantomData,
    };

    unsafe {
        target.clone_from_spec(&source);
    }

    assert_eq!(target.table.items, source.table.items);
    assert_eq!(target.table.growth_left, source.table.growth_left);
}

