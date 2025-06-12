// Answer 0

#[test]
fn test_clear_no_drop_empty() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = MockAllocator;
    let mut table: RawTable<u32, MockAllocator> = RawTable::new_in(allocator);
    
    assert!(table.is_empty());
    
    table.clear_no_drop(); // Verify that it doesn't panic on empty table
}

#[test]
fn test_clear_no_drop_non_empty() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = MockAllocator;
    let mut table: RawTable<u32, MockAllocator> = RawTable::with_capacity_in(4, allocator);

    unsafe {
        let hash = 1;
        table.insert(hash, 10, |v| *v);
    }

    assert!(!table.is_empty());
    table.clear_no_drop(); // Verify that it doesn't panic and items are not dropped
    assert_eq!(table.len(), 0); // Ensure the length is zero after clearing
}

