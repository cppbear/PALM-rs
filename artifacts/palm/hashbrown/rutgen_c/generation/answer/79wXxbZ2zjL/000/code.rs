// Answer 0

#[test]
fn test_allocator() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for test
        }
    }

    let alloc = TestAllocator;
    let table: RawTable<i32, TestAllocator> = RawTable::new_in(alloc);
    
    assert!(table.allocator() as *const _ == &alloc as *const _);
}

#[test]
fn test_allocator_with_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for test
        }
    }

    let alloc = TestAllocator;
    let capacity = 10;
    let table: RawTable<i32, TestAllocator> = RawTable::with_capacity_in(capacity, alloc);
    
    assert!(table.allocator() as *const _ == &alloc as *const _);
}

