// Answer 0

#[test]
fn test_clone_from_spec() {
    use core::alloc::{Global, Layout};
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let ptr = core::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            core::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let mut allocator = TestAllocator;

    let source = RawTable {
        table: RawTableInner {
            bucket_mask: 0b1111,
            ctrl: NonNull::new_unchecked(allocator.allocate(Layout::from_size_align(16, 8).unwrap()).unwrap().as_ptr()),
            growth_left: 10,
            items: 5,
        },
        alloc: allocator,
        marker: PhantomData,
    };

    let mut target = RawTable {
        table: RawTableInner {
            bucket_mask: 0b1111,
            ctrl: NonNull::new_unchecked(allocator.allocate(Layout::from_size_align(16, 8).unwrap()).unwrap().as_ptr()),
            growth_left: 0,
            items: 0,
        },
        alloc: allocator,
        marker: PhantomData,
    };

    unsafe {
        target.clone_from_spec(&source);
    }

    assert_eq!(target.table.items, source.table.items);
    assert_eq!(target.table.growth_left, source.table.growth_left);
}

