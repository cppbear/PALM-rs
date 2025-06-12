// Answer 0

#[derive(Clone)]
struct TestAllocator;

impl Allocator for TestAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
        // Simulate successful allocation regardless of layout
        let ptr = unsafe { NonNull::new(alloc(layout.size()).expect("Allocation failed")).expect("Allocation returned null") };
        Ok(ptr)
    }
}

#[test]
fn test_do_alloc_success() {
    let allocator = TestAllocator;
    let layout = Layout::from_size_align(64, 8).expect("Failed to create layout");
    
    let result = do_alloc(&allocator, layout);
    assert!(result.is_ok());
}

#[test]
fn test_do_alloc_zero_size() {
    let allocator = TestAllocator;
    let layout = Layout::from_size_align(0, 8).expect("Failed to create layout");
    
    let result = do_alloc(&allocator, layout);
    assert!(result.is_ok());
}  

#[test]
fn test_do_alloc_large_size() {
    struct LargeAllocator;

    impl Allocator for LargeAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            if layout.size() > 1024 {
                return Err(());
            }
            Ok(unsafe { NonNull::new(alloc(layout.size())).expect("Allocation failed") })
        }
    }

    let allocator = LargeAllocator;
    let layout = Layout::from_size_align(1024, 8).expect("Failed to create layout");

    let result = do_alloc(&allocator, layout);
    assert!(result.is_ok());
}  

#[test]
#[should_panic]
fn test_do_alloc_fail() {
    struct FailingAllocator;

    impl Allocator for FailingAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
    }

    let allocator = FailingAllocator;
    let layout = Layout::from_size_align(64, 8).expect("Failed to create layout");

    let result = do_alloc(&allocator, layout);
    assert!(result.is_err());
}  

