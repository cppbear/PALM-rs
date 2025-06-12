// Answer 0

#[test]
fn test_do_alloc_err_case_zero_size() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let alloc = DummyAllocator;
    let layout = Layout::from_size_align(0, 1).unwrap();
    let _ = do_alloc(&alloc, layout);
}

#[test]
fn test_do_alloc_err_case_small_size() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let alloc = DummyAllocator;
    let layout = Layout::from_size_align(1, 1).unwrap();
    let _ = do_alloc(&alloc, layout);
}

#[test]
fn test_do_alloc_err_case_max_size() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let alloc = DummyAllocator;
    let layout = Layout::from_size_align(usize::MAX, 1).unwrap();
    let _ = do_alloc(&alloc, layout);
}

#[test]
fn test_do_alloc_err_case_invalid_alignment() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let alloc = DummyAllocator;
    let layout = Layout::from_size_align(1, usize::MAX).unwrap();
    let _ = do_alloc(&alloc, layout);
}

