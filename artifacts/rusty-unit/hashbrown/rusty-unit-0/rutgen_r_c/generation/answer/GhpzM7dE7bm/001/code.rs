// Answer 0

#[test]
fn test_try_insert_vacant_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    let result = map.try_insert(37, "a");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &mut "a");

    let next_result = map.try_insert(37, "b");
    assert!(next_result.is_err());
    if let Err(OccupiedError { entry, value }) = next_result {
        assert_eq!(entry.key(), &37);
        assert_eq!(entry.get(), &"a");
        assert_eq!(value, "b");
    }
}

#[test]
fn test_try_insert_multiple_vacant_entries() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    let result = map.try_insert("key1".to_string(), 10);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &mut 10);

    let result2 = map.try_insert("key2".to_string(), 20);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), &mut 20);

    let occupied_error = map.try_insert("key1".to_string(), 30);
    assert!(occupied_error.is_err());
    if let Err(OccupiedError { entry, value }) = occupied_error {
        assert_eq!(entry.key(), &"key1".to_string());
        assert_eq!(entry.get(), &10);
        assert_eq!(value, 30);
    }
}

#[test]
fn test_try_insert_boundary_case() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    // Insert a maximum of 10 entries to avoid large allocations
    for i in 0..10 {
        let result = map.try_insert(i, "value");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &mut "value");
    }

    // Verify error when trying to re-insert an existing key
    let occupied_error = map.try_insert(5, "new_value");
    assert!(occupied_error.is_err());
    if let Err(OccupiedError { entry, value }) = occupied_error {
        assert_eq!(entry.key(), &5);
        assert_eq!(entry.get(), &"value");
        assert_eq!(value, "new_value");
    }
}

