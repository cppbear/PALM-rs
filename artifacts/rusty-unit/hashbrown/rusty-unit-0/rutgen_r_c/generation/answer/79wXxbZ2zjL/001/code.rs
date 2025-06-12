// Answer 0

#[test]
fn test_allocator() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No operation
        }
    }

    let alloc = DummyAllocator;
    let raw_table: RawTable<i32, DummyAllocator> = RawTable::new_in(alloc);
    
    let allocator_ref = raw_table.allocator();
    assert!(std::ptr::eq(allocator_ref as *const _, &alloc as *const _));
}

#[test]
fn test_allocator_with_capacity() {
    struct AnotherAllocator;

    unsafe impl Allocator for AnotherAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No operation
        }
    }

    let alloc = AnotherAllocator;
    let raw_table: RawTable<i32, AnotherAllocator> = RawTable::with_capacity_in(10, alloc);
    
    let allocator_ref = raw_table.allocator();
    assert!(std::ptr::eq(allocator_ref as *const _, &alloc as *const _));
}

