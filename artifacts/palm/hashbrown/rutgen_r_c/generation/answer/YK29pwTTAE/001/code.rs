// Answer 0

fn test_contains_key_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHasher, MyAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    map.insert(1, "a");
    
    assert_eq!(map.contains_key(&1), true);
}

fn test_contains_key_non_existing_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHasher, MyAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    map.insert(1, "a");

    assert_eq!(map.contains_key(&2), false);
}

fn test_contains_key_borrowed_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, i32, DefaultHasher, MyAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    map.insert("key".to_string(), 42);

    let borrowed_key: &str = "key";
    assert_eq!(map.contains_key(borrowed_key), true);
}

fn test_contains_key_empty_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHasher, MyAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    assert_eq!(map.contains_key(&1), false);
}

fn test_contains_key_large_key() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, i32, DefaultHasher, MyAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    let large_key = "a".repeat(1000); // very large key
    map.insert(large_key.clone(), 1);

    assert_eq!(map.contains_key(&large_key), true);
}

