// Answer 0

#[test]
fn test_search_none_case() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<u64, u64, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let builder = RawEntryBuilder { map: &map };

    let hash_value: u64 = 12345; // Arbitrary hash that is not present in the map
    let result = builder.search(hash_value, |key| *key == 67890); // No key should match
}

#[test]
fn test_search_none_case_with_empty_map() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<u64, u64, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let builder = RawEntryBuilder { map: &map };

    let hash_value: u64 = 0; // Testing the lower bound of the hash range
    let result = builder.search(hash_value, |key| false); // No key should match
}

#[test]
fn test_search_with_large_hash() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<u64, u64, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let builder = RawEntryBuilder { map: &map };

    let hash_value: u64 = u64::MAX; // Testing the upper bound of the hash range
    let result = builder.search(hash_value, |key| *key == 99999); // No key should match
}

