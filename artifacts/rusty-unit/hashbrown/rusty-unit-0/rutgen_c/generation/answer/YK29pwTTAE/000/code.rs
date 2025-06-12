// Answer 0

#[test]
fn test_contains_key_existing_key() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
}

#[test]
fn test_contains_key_non_existing_key() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, "a");
    assert_eq!(map.contains_key(&2), false);
}

#[test]
fn test_contains_key_empty_map() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    assert_eq!(map.contains_key(&1), false);
}

#[test]
fn test_contains_key_with_different_borrowed_type() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    map.insert(1, "a");
    let key: &dyn Hash = &1; // Simulating a borrowed type
    assert_eq!(map.contains_key(key), true);
}

