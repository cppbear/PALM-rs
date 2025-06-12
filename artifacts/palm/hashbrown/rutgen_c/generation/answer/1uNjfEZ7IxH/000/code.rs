// Answer 0

#[test]
fn test_get_many_mut_pointers_with_existing_buckets() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            todo!()
        }
    }

    let allocator = TestAllocator;
    let mut table = RawTable::<u32, TestAllocator>::new_in(allocator);

    // Assume we have inserted some elements into the table (mock the insertion here)
    // The hashes for the inserted elements
    let hashes = [1u64, 2, 3];
    let eq = |index: usize, value: &u32| {
        // Mock a condition that returns true for specific indices
        *value == (index as u32 + 1)
    };

    // Test calling `get_many_mut_pointers`
    unsafe {
        let results = table.get_many_mut_pointers(hashes, eq);
        for (i, result) in results.iter().enumerate() {
            match result {
                Some(ptr) => {
                    // Additional checks that can be performed on the valid pointer.
                    assert!(*ptr.as_ref() == (i as u32 + 1));
                }
                None => {
                    assert!(false, "Expected a valid pointer for index {}", i);
                }
            }
        }
    }
}

#[test]
fn test_get_many_mut_pointers_with_none() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            todo!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            todo!()
        }
    }

    let allocator = TestAllocator;
    let mut table = RawTable::<u32, TestAllocator>::new_in(allocator);

    // Suppose no elements are inserted, thus all should return None
    let hashes = [10u64, 11, 12];
    let eq = |_: usize, _: &u32| false; // Always return false to mock no matches

    unsafe {
        let results = table.get_many_mut_pointers(hashes, eq);
        for result in results.iter() {
            assert!(result.is_none(), "All results should be None");
        }
    }
}

