// Answer 0

#[test]
fn test_remove_existing_key() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(DummyAllocator),
    };

    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
}

#[test]
fn test_remove_non_existing_key() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(DummyAllocator),
    };

    assert_eq!(map.remove(&1), None);
    map.insert(2, "b");
    assert_eq!(map.remove(&1), None);
}

#[test]
fn test_remove_multiple_keys() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(DummyAllocator),
    };

    map.insert(1, "a");
    map.insert(2, "b");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&2), Some("b"));
    assert_eq!(map.remove(&1), None);
    assert_eq!(map.remove(&2), None);
}

#[test]
#[should_panic]
fn test_remove_panic_on_empty_map() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(DummyAllocator),
    };

    assert_eq!(map.remove(&1), None); // No panic expected.
}

