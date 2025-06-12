// Answer 0

#[test]
fn test_clone_from_impl() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Pretend to allocate and return a non-null pointer
            let ptr = NonNull::new_unchecked(std::alloc::alloc(layout));
            Ok(ptr)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            // Deallocate the memory
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    #[derive(Clone)]
    struct TestType {
        value: i32,
    }

    impl TestType {
        const NEEDS_DROP: bool = true;
    }

    let alloc = TestAllocator;
    let mut source_table = RawTable::<TestType, TestAllocator>::new_in(alloc);
    source_table.insert(1, TestType { value: 42 }, |v| v.value as u64);
    
    let mut clone_table = RawTable::<TestType, TestAllocator>::new_in(alloc);
    
    unsafe {
        clone_table.clone_from_impl(&source_table);
    }

    assert_eq!(clone_table.len(), source_table.len());
    assert_eq!(clone_table.get(1, |v| v.value == 42).is_some(), true);
}

