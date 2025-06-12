// Answer 0

#[test]
fn test_into_allocation_empty_singleton() {
    use core::alloc::{GlobalAlloc, Layout};
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    struct RawTableInner {
        bucket_mask: usize,
        ctrl: NonNull<u8>,
        growth_left: usize,
        items: usize,
    }

    struct RawTable<T, A: Allocator = TestAllocator> {
        table: RawTableInner,
        alloc: A,
        marker: PhantomData<T>,
    }

    impl RawTableInner {
        const NEW: Self = Self {
            bucket_mask: 0,
            ctrl: NonNull::dangling(),
            growth_left: 0,
            items: 0,
        };

        fn is_empty_singleton(&self) -> bool {
            self.bucket_mask == 0
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }
    }

    impl<T, A: Allocator> RawTable<T, A> {
        fn into_allocation(mut self) -> Option<(NonNull<u8>, Layout, A)> {
            let alloc = if self.table.is_empty_singleton() {
                None
            } else {
                let (layout, ctrl_offset) = match Self::TABLE_LAYOUT.calculate_layout_for(self.table.buckets()) {
                    Some(lco) => lco,
                    None => unsafe { core::hint::unreachable_unchecked() },
                };
                Some((
                    unsafe { NonNull::new_unchecked(self.table.ctrl.as_ptr().sub(ctrl_offset).cast()) },
                    layout,
                    unsafe { core::ptr::read(&self.alloc) },
                ))
            };
            core::mem::forget(self);
            alloc
        }
    }

    impl TableLayout {
        const fn new<T>() -> Self {
            let layout = Layout::new::<T>();
            Self {
                size: layout.size(),
                ctrl_align: if layout.align() > 16 { layout.align() } else { 16 },
            }
        }
    }

    let table = RawTableInner::NEW;
    let raw_table = RawTable {
        table,
        alloc: TestAllocator,
        marker: PhantomData::<u8>,
    };

    let result = raw_table.into_allocation();
    assert!(result.is_none());
}

