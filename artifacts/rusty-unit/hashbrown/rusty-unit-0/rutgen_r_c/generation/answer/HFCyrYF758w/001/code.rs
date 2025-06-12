// Answer 0

#[test]
fn test_is_bucket_full_valid() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let capacity = 4; // Choose a capacity that is a power of two
    let table: RawTable<u8, TestAllocator> = RawTable::with_capacity_in(capacity, allocator);
    
    unsafe {
        // Assuming we have a method to fill a bucket
        assert!(!table.is_bucket_full(0)); // Initial state should not be full

        // Simulate filling the bucket (a method to fill would be needed here)
        // fill_bucket(&table, 0); // This line is conceptual and represents filling the bucket.

        assert!(table.is_bucket_full(0)); // Now it should be full after filling
    }
}

#[test]
#[should_panic]
fn test_is_bucket_full_out_of_bounds() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let capacity = 4; // Choose a capacity that is a power of two
    let table: RawTable<u8, TestAllocator> = RawTable::with_capacity_in(capacity, allocator);
    
    unsafe {
        // This should panic as it's out of bounds
        let _ = table.is_bucket_full(4); // Since we only have buckets 0 to 3 (4 buckets total)
    }
}

#[test]
fn test_is_bucket_full_empty() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let capacity = 4; // Choose a capacity that is a power of two
    let table: RawTable<u8, TestAllocator> = RawTable::with_capacity_in(capacity, allocator);

    unsafe {
        // Check all buckets should be empty initially
        for i in 0..capacity {
            assert!(!table.is_bucket_full(i)); // Should not be full
        }
    }
}

