// Answer 0

#[test]
fn test_do_alloc_success() {
    use std::alloc::{Allocator, Layout};
    use std::ptr::NonNull;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> std::alloc::Result<NonNull<u8>, ()> {
            let size = layout.size();
            if size > 0 {
                let layout = Layout::from_size_align(size, 1).unwrap();
                unsafe {
                    let ptr = std::alloc::alloc(layout) as *mut u8;
                    NonNull::new(ptr).ok_or(())
                }
            } else {
                Err(())
            }
        }

        fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            // Implement deallocation if needed for cleanup
        }
    }

    let alloc = TestAllocator;
    let layout = Layout::from_size_align(8, 1).unwrap();
    let result = do_alloc(&alloc, layout);

    assert!(result.is_ok());
}

