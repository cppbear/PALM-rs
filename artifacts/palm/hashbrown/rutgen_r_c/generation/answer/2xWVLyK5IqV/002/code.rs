// Answer 0

#[test]
fn test_or_insert_with_key_occupied_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct AllocatorImpl;

    impl Allocator for AllocatorImpl {
        fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: hashbrown::HashMap<String, usize, TestHasher, AllocatorImpl> = hashbrown::HashMap::new();
    map.insert("test_key".to_string(), 42);
    
    let entry_ref = map.entry_ref("test_key");
    let result = entry_ref.or_insert_with_key(|key| key.len());

    assert_eq!(*result, 42);
}

#[test]
fn test_or_insert_with_key_vacant_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct AllocatorImpl;

    impl Allocator for AllocatorImpl {
        fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: hashbrown::HashMap<String, usize, TestHasher, AllocatorImpl> = hashbrown::HashMap::new();
    
    {
        let entry_ref = map.entry_ref("new_key");
        let result = entry_ref.or_insert_with_key(|key| key.len());
        assert_eq!(*result, 7);
    }

    assert_eq!(map["new_key"], 7);
}

