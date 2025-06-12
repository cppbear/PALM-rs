// Answer 0

#[test]
fn test_search_found() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let hash_map: HashMap<i32, i32, std::collections::hash_map::RandomState, TestAllocator> = HashMap {
        hash_builder: std::collections::hash_map::RandomState::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    // Assume we have some data in the hash_map for testing
    // Typically, you would have some method to insert or setup initial data

    let builder = RawEntryBuilder { map: &hash_map };

    let result = builder.search(1, |&key| key == 10); // Assuming '10' exists in the table
    assert!(result.is_some());
}

#[test]
fn test_search_not_found() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let hash_map: HashMap<i32, i32, std::collections::hash_map::RandomState, TestAllocator> = HashMap {
        hash_builder: std::collections::hash_map::RandomState::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    // Assume we have some data in the hash_map for testing

    let builder = RawEntryBuilder { map: &hash_map };

    let result = builder.search(1, |&key| key == 999); // Assuming '999' does not exist in the table
    assert!(result.is_none());
}

