// Answer 0

#[test]
fn test_drain_empty_table() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    let drained: RawDrain<i32, TestAllocator> = table.drain();
    assert_eq!(drained.iter.items, 0);
}

#[test]
fn test_drain_populated_table() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::with_capacity_in(8, allocator);
    
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
    }

    let drained: RawDrain<i32, TestAllocator> = table.drain();
    assert!(drained.iter.items > 0);
}

