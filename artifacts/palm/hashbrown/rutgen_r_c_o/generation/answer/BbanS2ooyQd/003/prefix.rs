// Answer 0

#[test]
fn test_shrink_to_with_zero_min_size() {
    use core::alloc::{Global, Layout};
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(8, allocator);
    let hasher = |&x: &u64| x.wrapping_mul(31);

    table.shrink_to(0, hasher);
}

#[test]
fn test_shrink_to_with_buckets() {
    use core::alloc::{Global, Layout};
    use core::ptr::NonNull;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(16, allocator);
    let hasher = |&x: &u64| x.wrapping_mul(31);

    // Fill the table with items
    for i in 0..5 {
        let _bucket = table.insert(i.wrapping_mul(10), i, hasher);
    }

    // Check the conditions are met
    assert!(table.buckets() > 4); // ensure self.buckets() > buckets
    table.shrink_to(3, hasher);
}

