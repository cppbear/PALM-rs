// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };
    
    map.insert(1, "a");
    assert_eq!(map.remove_entry(&1), Some((1, "a")));
    assert!(map.remove(&1).is_none());
}

#[test]
fn test_remove_entry_non_existing_key() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    assert!(map.remove_entry(&1).is_none());
}

#[test]
#[should_panic]
fn test_remove_entry_with_invalid_key_type() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, "a");

    // Attempting to remove with a different type (e.g., string)
    let _ = map.remove_entry(&"1");
}

#[test]
fn test_remove_entry_edge_case() {
    struct SimpleAllocator;

    impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: SimpleAllocator,
            marker: PhantomData,
        },
    };
    
    assert!(map.remove_entry(&0).is_none()); // Removing from empty map
    
    map.insert(2, "b");
    assert_eq!(map.remove_entry(&2), Some((2, "b"))); // Removing existing key again
}

