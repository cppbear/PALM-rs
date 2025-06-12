// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(TestAllocator),
    };
    
    map.insert(1, "a");
    
    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
}

#[test]
fn test_get_key_value_non_existing_key() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(TestAllocator),
    };

    assert_eq!(map.get_key_value(&2), None);
}

#[test]
fn test_get_key_value_with_different_reference_type() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(TestAllocator),
    };
    
    map.insert(3, "b");
    
    let key: &i32 = &3; // borrowed reference
    assert_eq!(map.get_key_value(key), Some((&3, &"b")));
}

