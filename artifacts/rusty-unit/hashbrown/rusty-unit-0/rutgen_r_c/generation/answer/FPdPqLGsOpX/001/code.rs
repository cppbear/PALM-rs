// Answer 0

#[test]
fn test_do_alloc_with_error() {
    struct MockAllocator;

    impl Allocator for MockAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let layout = Layout::from_size_align(0, 1).unwrap(); // minimal size layout
    let allocator = MockAllocator;

    let result = do_alloc(&allocator, layout);
    
    assert_eq!(result, Err(()));
}

