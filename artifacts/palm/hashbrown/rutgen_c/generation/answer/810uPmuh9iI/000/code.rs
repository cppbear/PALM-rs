// Answer 0

#[test]
fn test_clear_empty_table() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let alloc = DummyAllocator;
    let mut table: RawTable<i32, DummyAllocator> = RawTable::new_in(alloc);

    table.clear();
    
    assert!(table.is_empty());
}

#[test]
fn test_clear_non_empty_table() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let alloc = DummyAllocator;
    let mut table: RawTable<i32, DummyAllocator> = RawTable::with_capacity_in(10, alloc);

    // Simulate adding an element to the table
    unsafe {
        table.insert(1, 42, |v| *v);
    }
    
    assert!(!table.is_empty());

    table.clear();
    
    assert!(table.is_empty());
}

