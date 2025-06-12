// Answer 0

fn test_search_found() {
    use std::collections::HashMap as StdHashMap;
    use std::hash::{BuildHasher, Hasher, Hash};
    
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = Self;
        
        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }
    
    impl Hasher for TestHasher {
        fn write(&mut self, _bytes: &[u8]) {}
        
        fn finish(&self) -> u64 {
            42 // Fixed value for testing
        }
    }

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let map: HashMap<i32, &str, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: std::marker::PhantomData,
        },
    };

    let entry_builder = RawEntryBuilder {
        map: &map,
    };

    let hash: u64 = 42; // A hash value that matches the key
    let is_match = |k: &i32| *k == 1; // Match function that finds the key 1
    let result = entry_builder.search(hash, is_match);
    
    assert_eq!(result, Some((&1, &"value"))); // Expected result for the test
}

fn test_search_not_found() {
    use std::collections::HashMap as StdHashMap;
    use std::hash::{BuildHasher, Hasher, Hash};

    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = Self;

        fn build_hasher(&self) -> Self::Hasher {
            TestHasher
        }
    }

    impl Hasher for TestHasher {
        fn write(&mut self, _bytes: &[u8]) {}

        fn finish(&self) -> u64 {
            42 // Fixed hash value for this test
        }
    }

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let map: HashMap<i32, &str, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: std::marker::PhantomData,
        },
    };

    let entry_builder = RawEntryBuilder {
        map: &map,
    };

    let hash: u64 = 999; // A hash value that does not match any key
    let is_match = |k: &i32| *k == 1; // Match function that finds the key 1
    let result = entry_builder.search(hash, is_match);

    assert_eq!(result, None); // Expected result for this test
}

