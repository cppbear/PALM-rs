// Answer 0

#[test]
fn test_with_capacity_in() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(0, 1).unwrap())))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No operation needed for the dummy allocator
        }
    }

    let alloc = DummyAllocator;
    let capacity = 16;
    let table: RawTable<i32, DummyAllocator> = RawTable::with_capacity_in(capacity, alloc);

    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= capacity);
}

#[test]
fn test_with_capacity_in_zero() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(0, 1).unwrap())))
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No operation needed for the dummy allocator
        }
    }

    let alloc = DummyAllocator;
    let capacity = 0;
    let table: RawTable<i32, DummyAllocator> = RawTable::with_capacity_in(capacity, alloc);

    assert_eq!(table.len(), 0);
    assert!(table.capacity() == 0);
}

