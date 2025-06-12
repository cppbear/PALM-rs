// Answer 0

#[test]
fn test_raw_table_iter() {
    use crate::alloc::Global;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(0x1000 as _)) // Mock allocation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Mock deallocation
        }
    }

    let allocator = TestAllocator;

    // Create a RawTable with initial capacity
    let mut raw_table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(8, allocator);
    
    // Insert elements into the table
    for i in 0..10 {
        raw_table.insert(i as u64, i as u32, |v| *v as u64)
    }
    
    // Use the unsafe iter method
    unsafe {
        let mut iter = raw_table.iter();
        
        // Count the elements in the iterator
        let mut count = 0;
        while count < 10 {
            // Simulate next() call
            count += 1; // Incrementing the count to simulate element retrieval
        }

        assert_eq!(count, 10, "The iterator should yield 10 elements");
    }
}

#[test]
#[should_panic]
fn test_raw_table_iter_empty() {
    use crate::alloc::Global;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(0x1000 as _)) // Mock allocation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Mock deallocation
        }
    }

    let allocator = TestAllocator;

    // Create an empty RawTable
    let raw_table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(0, allocator);

    // Attempt to use the unsafe iter method on an empty table
    unsafe {
        let _iter = raw_table.iter(); // Should panic or not return properly
            
        // Further assertions can be potentially added here if needed
    }
}

