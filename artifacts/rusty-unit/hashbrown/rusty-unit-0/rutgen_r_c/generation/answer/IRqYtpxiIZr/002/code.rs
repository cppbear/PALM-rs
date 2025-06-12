// Answer 0

#[test]
fn test_get_key_value_not_found() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(), 
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let result = map.get_key_value(&2);
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_empty_map() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable {
            table: RawTableInner::default(), 
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let result = map.get_key_value(&1);
    assert_eq!(result, None);
}

