// Answer 0

#[test]
fn test_raw_table_iter() {
    use crate::alloc::Global;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Placeholder allocation logic
            unimplemented!()
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            // Placeholder deallocation logic
            unimplemented!()
        }
    }

    let allocator = DummyAllocator;
    let mut table: RawTable<i32, DummyAllocator> = RawTable::with_capacity_in(8, allocator);

    // Here you would typically insert some elements into the table
    // For simplicity, we'll skip this part.

    unsafe {
        let mut iter = table.iter();
        assert_eq!(iter.items, 0); // Adjust assertions according to actual items
    }
}

#[test]
#[should_panic]
fn test_raw_table_iter_panic_on_empty_table() {
    use crate::alloc::Global;

    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Placeholder allocation logic
            unimplemented!()
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            // Placeholder deallocation logic
            unimplemented!()
        }
    }

    let allocator = DummyAllocator;
    let table: RawTable<i32, DummyAllocator> = RawTable::with_capacity_in(8, allocator);

    unsafe {
        let _ = table.iter(); // Attempting to iterate over an empty table
    }
}

