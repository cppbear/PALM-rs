// Answer 0

#[test]
fn test_bucket_valid_index() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    
    // Assume we have allocated space for at least 4 buckets.
    table.table.bucket_mask = 3; // 4 buckets (0 to 3)

    unsafe {
        let bucket = table.bucket(0);
        assert_eq!(bucket.as_ptr() as usize, table.data_end().as_ptr() as usize);
        
        let bucket = table.bucket(3);
        assert_eq!(bucket.as_ptr() as usize, table.data_end().as_ptr() as usize);
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bucket_index_out_of_bounds() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    
    // Assume we have allocated space for at least 4 buckets.
    table.table.bucket_mask = 3; // 4 buckets (0 to 3)

    unsafe {
        // This should panic as we're trying to access bucket index 4 which is out of bounds
        let _bucket = table.bucket(4);
    }
} 

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bucket_zero_size() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    
    // Set bucket_mask to 0 for zero-size check
    table.table.bucket_mask = 0; // No valid buckets

    unsafe {
        // Valid access for zero size but no valid buckets should panic
        let _bucket = table.bucket(0);
    }
}

