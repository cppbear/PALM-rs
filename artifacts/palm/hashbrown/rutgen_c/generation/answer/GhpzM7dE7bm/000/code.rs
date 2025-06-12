// Answer 0

#[test]
fn test_try_insert_new_key() {
    struct TestAllocator;

    // Implement the Allocator trait for the test allocator
    // (Providing stub implementations since they aren't actually used for the test)
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap::new();
    assert_eq!(map.try_insert(1, "hello").unwrap(), &"hello");
    assert_eq!(map.try_insert(2, "world").unwrap(), &"world");
    assert_eq!(map.try_insert(1, "another").is_err(), true);
}

#[test]
fn test_try_insert_existing_key() {
    struct TestAllocator;

    // Implement the Allocator trait for the test allocator
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.try_insert(1, "hello").unwrap();

    match map.try_insert(1, "world") {
        Err(OccupiedError { entry, value }) => {
            assert_eq!(entry.key(), &1);
            assert_eq!(value, "world");
        }
        _ => panic!("Expected an error when inserting an existing key"),
    }
}

