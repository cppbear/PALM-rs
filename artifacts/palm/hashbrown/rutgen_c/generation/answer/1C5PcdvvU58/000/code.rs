// Answer 0

#[test]
fn test_keys_empty() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {}
    }

    let map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), TestAllocator);
    let keys: Vec<_> = map.keys().collect();
    assert!(keys.is_empty());
}

#[test]
fn test_keys_single_element() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), TestAllocator);
    map.insert("a", 1);
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec!["a"]);
}

#[test]
fn test_keys_multiple_elements() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _: std::ptr::NonNull<u8>, _: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::default(), TestAllocator);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let mut keys: Vec<_> = map.keys().collect();
    
    keys.sort_unstable();
    assert_eq!(keys, vec!["a", "b", "c"]);
}

