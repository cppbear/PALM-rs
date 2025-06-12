// Answer 0

#[derive(Default)]
struct TestAllocator;

impl Allocator for TestAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
        if layout.size() >= 1 && layout.size() <= 1024 {
            let ptr = unsafe { NonNull::new(alloc(layout)).ok_or(())? };
            Ok(ptr)
        } else {
            Err(())
        }
    }

    // Implement the deallocate method as required by the Allocator trait.
    fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Implementation not needed for test, can be a no-op
    }
}

#[test]
fn test_do_alloc_valid_size() {
    let allocator = TestAllocator::default();
    let layout = Layout::from_size_align(32, 1).unwrap();
    let _ = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_edge_case_min_size() {
    let allocator = TestAllocator::default();
    let layout = Layout::from_size_align(1, 1).unwrap();
    let _ = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_edge_case_max_size() {
    let allocator = TestAllocator::default();
    let layout = Layout::from_size_align(1024, 1).unwrap();
    let _ = do_alloc(&allocator, layout);
}

#[test]
#[should_panic]
fn test_do_alloc_too_small_size() {
    let allocator = TestAllocator::default();
    let layout = Layout::from_size_align(0, 1).unwrap(); // 0 bytes
    let _ = do_alloc(&allocator, layout);
}

#[test]
#[should_panic]
fn test_do_alloc_too_large_size() {
    let allocator = TestAllocator::default();
    let layout = Layout::from_size_align(2048, 1).unwrap(); // exceeds max size
    let _ = do_alloc(&allocator, layout);
}

