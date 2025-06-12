// Answer 0

fn test_insert_new_key() {
    // Arrange
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    // Act
    let result = map.insert(37, "a");

    // Assert
    assert_eq!(result, None);
    assert!(!map.is_empty());
}

fn test_insert_existing_key() {
    // Arrange
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    map.insert(37, "a");

    // Act
    let result = map.insert(37, "b");

    // Assert
    assert_eq!(result, Some("a"));
    assert_eq!(map.get(&37), Some(&"b"));
}

fn test_insert_multiple_keys() {
    // Arrange
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    // Act
    let result1 = map.insert(37, "a");
    let result2 = map.insert(38, "b");
    let result3 = map.insert(37, "c");

    // Assert
    assert_eq!(result1, None);
    assert_eq!(result2, None);
    assert_eq!(result3, Some("a"));
    assert_eq!(map.get(&37), Some(&"c"));
    assert_eq!(map.get(&38), Some(&"b"));
}

fn test_insert_with_different_keys() {
    // Arrange
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::new(),
            alloc: DummyAllocator,
            marker: PhantomData,
        },
    };

    // Act
    let result1 = map.insert(50, "x");
    let result2 = map.insert(200, "y");
    let result3 = map.insert(50, "z");

    // Assert
    assert_eq!(result1, None);
    assert_eq!(result2, None);
    assert_eq!(result3, Some("x"));
    assert_eq!(map.get(&50), Some(&"z"));
    assert_eq!(map.get(&200), Some(&"y"));
}

