// Answer 0

#[test]
fn test_insert_new_element() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);
    let hash = 42;
    let value = 7;

    let bucket = table.insert(hash, value, |value| *value as u64);
    assert!(!bucket.ptr.is_null());
}

#[test]
fn test_insert_replaces_tombstone() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    // Simulate inserting a value and then deleting it (making it a tombstone).
    let hash = 42;
    let value = 7;
    let _bucket = table.insert(hash, value, |value| *value as u64);
    
    // Assume some mechanism to simulate a tombstone, e.g., clearing out the previous bucket
    let tombstone_value = -1;
    let _tombstone_bucket = table.insert(hash, tombstone_value, |value| *value as u64);

    // Now insert a new value that should replace the tombstone
    let new_value = 8;
    let bucket = table.insert(hash, new_value, |value| *value as u64);
    assert!(!bucket.ptr.is_null());
}

#[should_panic]
#[test]
fn test_insert_fails_when_table_is_full() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(allocator);

    // Simulate filling the table to its capacity and attempting to insert one more element
    for i in 0..1024 {
        table.insert(i as u64, i, |value| *value as u64);
    }

    // This should trigger a panic as the table is at capacity.
    table.insert(1025, 1025, |value| *value as u64);
}

