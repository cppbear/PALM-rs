// Answer 0

#[test]
fn test_get_with_non_existent_hash() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut raw_table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    let result = raw_table.get(12345, |value| *value == 10);
    assert!(result.is_none());
}

#[test]
fn test_get_with_empty_table() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut raw_table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    let result = raw_table.get(0, |value| *value == 0);
    assert!(result.is_none());
}

