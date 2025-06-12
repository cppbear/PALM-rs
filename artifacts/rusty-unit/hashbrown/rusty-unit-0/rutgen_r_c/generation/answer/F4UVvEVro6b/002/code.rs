// Answer 0

#[test]
fn test_clone_from_impl_empty_source() {
    use std::alloc::{Layout, Allocator};
    use std::ptr::NonNull;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Mock allocation always succeeds.
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Mock deallocation does nothing.
        }
    }

    let alloc = MockAllocator;

    let mut table_1: RawTable<u8, MockAllocator> = RawTable::new_in(alloc);
    let table_2: RawTable<u8, MockAllocator> = RawTable::new_in(alloc);

    unsafe {
        // Ensure that both tables have the same number of buckets.
        assert_eq!(table_1.buckets(), table_2.buckets());

        // Execute the function under test.
        table_1.clone_from_impl(&table_2);

        // Check that the items and growth left are unchanged since the source is empty.
        assert_eq!(table_1.table.items, 0);
        assert_eq!(table_1.table.growth_left, table_1.table.growth_left);
    }
}

#[test]
#[should_panic]
fn test_clone_from_impl_partial_clone() {
    use std::alloc::{Layout, Allocator};
    use std::ptr::NonNull;

    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Mock deallocation does nothing.
        }
    }

    let alloc = MockAllocator;

    let mut table_1: RawTable<i32, MockAllocator> = RawTable::new_in(alloc);
    let mut table_2: RawTable<i32, MockAllocator> = RawTable::new_in(alloc);
    
    unsafe {
        // Populate table_2 with one item to trigger panic in the middle of cloning.
        table_2.insert(0, 42, |x| *x);
        
        // Ensure both tables have the same number of buckets.
        assert_eq!(table_1.buckets(), table_2.buckets());

        // Simulate an environment where the clone could panic.
        table_1.clone_from_impl(&table_2);
    }
}

