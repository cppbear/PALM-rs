// Answer 0

#[test]
fn test_build_hashes_inner_valid_range() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        fn build_hasher(&self) -> std::hash::Hasher {
            // A simple hasher implementation for testing
            let hasher = std::collections::hash_map::DefaultHasher::new();
            hasher
        }
    }

    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, String, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: RawTableInner,
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&i32; 3] = [&1, &2, &3];
    let hashes = map.build_hashes_inner(keys);
}

#[test]
#[should_panic]
fn test_build_hashes_inner_invalid_index() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        fn build_hasher(&self) -> std::hash::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, String, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: RawTableInner,
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&i32; 0] = []; // should panic on invalid N
    let hashes = map.build_hashes_inner(keys);
}

#[test]
fn test_build_hashes_inner_large_input() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        fn build_hasher(&self) -> std::hash::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, String, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: RawTableInner,
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&i32; 100] = [&0; 100]; // All keys are zero
    let hashes = map.build_hashes_inner(keys);
}

