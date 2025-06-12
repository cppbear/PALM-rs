// Answer 0

#[test]
fn test_build_hashes_inner_zero_items() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = TestHasher;
    let mut hashmap: HashMap<i32, String, _, TestAllocator> = HashMap {
        hash_builder: hasher,
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };
    
    let hashes: [u64; 0] = hashmap.build_hashes_inner([]);
}

#[test]
fn test_build_hashes_inner_one_item() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = TestHasher;
    let mut hashmap: HashMap<i32, String, _, TestAllocator> = HashMap {
        hash_builder: hasher,
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&i32; 1] = [&1];
    let hashes = hashmap.build_hashes_inner(keys);
}

#[test]
fn test_build_hashes_inner_two_items() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = TestHasher;
    let mut hashmap: HashMap<i32, String, _, TestAllocator> = HashMap {
        hash_builder: hasher,
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&i32; 2] = [&1, &2];
    let hashes = hashmap.build_hashes_inner(keys);
}

#[test]
fn test_build_hashes_inner_ten_items() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = TestHasher;
    let mut hashmap: HashMap<i32, String, _, TestAllocator> = HashMap {
        hash_builder: hasher,
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&i32; 10] = [&0, &1, &2, &3, &4, &5, &6, &7, &8, &9];
    let hashes = hashmap.build_hashes_inner(keys);
}

