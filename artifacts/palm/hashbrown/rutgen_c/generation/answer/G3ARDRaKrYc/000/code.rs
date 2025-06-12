// Answer 0

#[test]
fn test_remove_entry_existing_element() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Mock implementation
            NonNull::new_unchecked(Box::into_raw(Box::new(0)))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Mock implementation
        }
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(1, 42, |&x| x);
    
    let removed = table.remove_entry(1, |&x| x == 42);
    
    assert_eq!(removed, Some(42));
    assert!(table.is_empty());
}

#[test]
fn test_remove_entry_nonexistent_element() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Mock implementation
            NonNull::new_unchecked(Box::into_raw(Box::new(0)))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Mock implementation
        }
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(1, 42, |&x| x);
    
    let removed = table.remove_entry(2, |&x| x == 99);
    
    assert_eq!(removed, None);
    assert_eq!(table.len(), 1);
}

