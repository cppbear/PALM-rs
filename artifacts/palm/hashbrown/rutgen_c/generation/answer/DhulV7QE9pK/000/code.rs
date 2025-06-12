// Answer 0

#[test]
fn test_insert_new_key() {
    struct SimpleAllocator;
    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.insert(37, "b"), Some("a"));
}

#[test]
fn test_insert_duplicate_key() {
    struct SimpleAllocator;
    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    map.insert(37, "a");
    assert_eq!(map.insert(37, "b"), Some("a"));
    assert_eq!(map.insert(37, "c"), Some("b"));
}

#[test]
fn test_insert_empty_map() {
    struct SimpleAllocator;
    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    assert_eq!(map.insert(42, "value"), None);
    assert_eq!(map.get(&42), Some(&"value"));
}

#[test]
fn test_insert_multiple_keys() {
    struct SimpleAllocator;
    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    assert_eq!(map.get(&1), Some(&"one"));
    assert_eq!(map.get(&2), Some(&"two"));
    assert_eq!(map.get(&3), Some(&"three"));
}

