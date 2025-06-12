// Answer 0

#[test]
fn test_raw_table_new_in() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8))))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            Box::from_raw(ptr.as_ptr());
        }
    }

    let allocator = TestAllocator;
    let table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    assert_eq!(table.len(), 0);
    assert!(table.capacity() >= 1);
}

