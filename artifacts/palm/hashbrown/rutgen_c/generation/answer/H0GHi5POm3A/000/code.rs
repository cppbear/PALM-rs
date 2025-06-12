// Answer 0

#[test]
fn test_get_existing_key() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
}

#[test]
fn test_get_non_existing_key() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };
    assert_eq!(map.get(&2), None);
}

#[test]
fn test_get_with_different_type() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };
    map.insert(1, "a");
    assert_eq!(map.get(&(1 as i32)), Some(&"a"));
}

#[test]
fn test_get_from_empty_map() {
    struct DummyAllocator;
    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };
    assert_eq!(map.get(&1), None);
}

