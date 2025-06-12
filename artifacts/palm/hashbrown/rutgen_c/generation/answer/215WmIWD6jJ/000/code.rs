// Answer 0

#[test]
fn test_raw_table_new() {
    use crate::alloc::Layout;
    
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let table: RawTable<u8, TestAllocator> = RawTable::new();
    assert_eq!(table.table.bucket_mask, 0); // Assuming bucket_mask is initialized to 0
    assert_eq!(table.table.growth_left, 0); // Assuming growth_left is initialized to 0
    assert_eq!(table.table.items, 0); // Assuming items is initialized to 0
}

#[test]
fn test_raw_table_with_capacity() {
    let capacity = 10;
    let table: RawTable<u8> = RawTable::with_capacity(capacity);
    assert_eq!(table.table.bucket_mask, 0); // Depending on implementation, adjust as necessary
    assert_eq!(table.table.growth_left, 10); // Assuming growth_left starts as the given capacity
    assert_eq!(table.table.items, 0); // Assuming items is initialized to 0
}

