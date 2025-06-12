// Answer 0

#[test]
fn test_into_key() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let entry = VacantEntry {
        hash: 0,
        key: "test_key",
        table: &mut map,
    };

    let key = entry.into_key();
    assert_eq!(key, "test_key");
}

#[test]
fn test_into_key_empty_string() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new_unchecked(std::ptr::null_mut()))
        }
        
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let entry = VacantEntry {
        hash: 0,
        key: "",
        table: &mut map,
    };

    let key = entry.into_key();
    assert_eq!(key, "");
}

