// Answer 0

#[test]
fn test_do_alloc_success() {
    use std::alloc::{GlobalAlloc, Layout};
    use std::ptr::NonNull;
    
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // For testing, we always succeed
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                Err(())
            } else {
                Ok(NonNull::new(ptr).unwrap())
            }
        }
    }

    let layout = Layout::from_size_align(16, 8).unwrap();
    let allocator = DummyAllocator;
    let result = do_alloc(&allocator, layout);
    assert!(result.is_ok());
}

#[test]
fn test_do_alloc_failure() {
    use std::alloc::{GlobalAlloc, Layout};
    use std::ptr::NonNull;

    struct FailAllocator;

    impl Allocator for FailAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let layout = Layout::from_size_align(16, 8).unwrap();
    let allocator = FailAllocator;
    let result = do_alloc(&allocator, layout);
    assert!(result.is_err());
}

