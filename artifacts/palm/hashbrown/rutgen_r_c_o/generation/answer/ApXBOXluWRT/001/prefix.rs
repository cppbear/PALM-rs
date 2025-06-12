// Answer 0

#[test]
fn test_fmt_with_default_allocator() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { std::alloc::alloc(layout) }).unwrap())
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let allocator = TestAllocator;
    let mut drain = Drain {
        inner: RawDrain {
            iter: RawIter::new(),
            table: RawTableInner::new(),
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };
    let _ = fmt(&drain, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { std::alloc::alloc(layout) }).unwrap())
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let allocator = CustomAllocator;
    let mut drain = Drain {
        inner: RawDrain {
            iter: RawIter::new(),
            table: RawTableInner::new(),
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };
    let _ = fmt(&drain, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_large_keys_and_values() {
    struct LargeAllocator;

    unsafe impl Allocator for LargeAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { std::alloc::alloc(layout) }).unwrap())
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let allocator = LargeAllocator;
    let data: Vec<(usize, usize)> = (0..100).map(|i| (i, i * 2)).collect();
    let mut drain = Drain {
        inner: RawDrain {
            iter: RawIter::new(),
            table: RawTableInner::new(),
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };
    let _ = fmt(&drain, &mut fmt::Formatter::new());
}

#[test]
#[should_panic]
fn test_fmt_with_empty_drain() {
    struct EmptyAllocator;

    unsafe impl Allocator for EmptyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(unsafe { std::alloc::alloc(layout) }).unwrap())
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout)
        }
    }

    let allocator = EmptyAllocator;
    let drain = Drain {
        inner: RawDrain {
            iter: RawIter::new(),
            table: RawTableInner::new(),
            orig_table: NonNull::dangling(),
            marker: PhantomData,
        },
    };
    let _ = fmt(&drain, &mut fmt::Formatter::new());
}

