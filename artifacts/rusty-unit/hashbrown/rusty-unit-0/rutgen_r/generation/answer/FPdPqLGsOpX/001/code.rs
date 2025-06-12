// Answer 0

fn do_alloc_test() {
    use std::alloc::{Allocator, Layout, Global};
    use std::ptr::NonNull;

    struct FailingAllocator;

    impl Allocator for FailingAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, std::alloc::AllocError> {
            Err(std::alloc::AllocError)
        }
    }

    #[test]
    fn test_do_alloc_with_failing_allocator() {
        let alloc = FailingAllocator;
        let layout = Layout::from_size_align(1, 1).unwrap();

        let result = do_alloc(&alloc, layout);
        assert_eq!(result, Err(()));
    }

    #[test]
    #[should_panic]
    fn test_do_alloc_with_invalid_layout() {
        let alloc = FailingAllocator;
        // Create a layout that is invalid (size is too large)
        let layout = Layout::from_size_align(usize::MAX, 1).unwrap_err();

        // This will panic; we should not expect a return value here
        let _ = do_alloc(&alloc, layout);
    }
}

