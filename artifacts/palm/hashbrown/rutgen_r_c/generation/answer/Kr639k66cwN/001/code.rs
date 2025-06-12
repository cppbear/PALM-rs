// Answer 0

#[test]
fn test_get_mut_existing_key() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }
    
    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    map.insert(1, "a");

    if let Some(value) = map.get_mut(&1) {
        *value = "b";
    }

    assert_eq!(map.get(&1), Some(&"b"));
}

#[test]
fn test_get_mut_non_existent_key() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    assert_eq!(map.get_mut(&2), None);
}

