// Answer 0

#[test]
fn test_get_many_mut_single_entry() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::new_unchecked(std::ptr::null_mut())) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    unsafe {
        table.insert(1, 100, |x| x);
    }
    let hashes = [1u64];
    let result = table.get_many_mut(hashes, |_, _| false);
}

#[test]
fn test_get_many_mut_multiple_entries() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::new_unchecked(std::ptr::null_mut())) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    unsafe {
        table.insert(1, 200, |x| x);
        table.insert(2, 300, |x| x);
        table.insert(3, 400, |x| x);
    }
    let hashes = [1u64, 2u64, 3u64];
    let result = table.get_many_mut(hashes, |_, _| false);
}

#[test]
fn test_get_many_mut_with_duplicate_hashes() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::new_unchecked(std::ptr::null_mut())) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    unsafe {
        table.insert(1, 500, |x| x);
        table.insert(2, 600, |x| x);
    }
    let hashes = [1u64, 1u64]; // Duplicate hashes
    let result = table.get_many_mut(hashes, |_, _| false);
}

#[test]
fn test_get_many_mut_empty_table() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::new_unchecked(std::ptr::null_mut())) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hashes = [1u64, 2u64]; // No entries in table
    let result = table.get_many_mut(hashes, |_, _| false);
}

#[test]
fn test_get_many_mut_exceeding_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::new_unchecked(std::ptr::null_mut())) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    unsafe {
        for i in 1..=10 {
            table.insert(i, 100 * i, |x| x);
        }
    }
    let hashes = [1u64, 2u64, 3u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64];
    let result = table.get_many_mut(hashes, |_, _| false);
}

