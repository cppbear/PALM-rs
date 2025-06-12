// Answer 0

#[test]
fn test_raw_table_new_in() {
    use crate::alloc::{Global, Layout};

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            // No deallocation needed for this test
        }
    }

    let allocator = TestAllocator;
    let table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);

    assert_eq!(table.buckets(), 1);
    assert!(table.is_empty());
}

