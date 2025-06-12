// Answer 0

#[test]
fn test_get_key_value_mut_existing_key() {
    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
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

    let (key, value) = map.get_key_value_mut(&1).unwrap();
    assert_eq!(key, &1);
    assert_eq!(value, &mut "a");
    *value = "b";

    assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "b")));
}

#[test]
fn test_get_key_value_mut_non_existent_key() {
    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::new(std::ptr::null_mut()).unwrap())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
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

    let result = map.get_key_value_mut(&2);
    assert!(result.is_none());
}

