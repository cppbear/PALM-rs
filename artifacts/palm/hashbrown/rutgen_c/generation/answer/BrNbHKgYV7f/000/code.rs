// Answer 0

#[test]
fn test_hasher_default() {
    struct MyAllocator;
    impl Allocator for MyAllocator {
        fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let hasher = DefaultHashBuilder::default();
    let map: HashMap<i32, i32, DefaultHashBuilder, MyAllocator> = HashMap::with_hasher_in(hasher, MyAllocator);
    assert_eq!(map.hasher() as *const _, &hasher as *const _);
}

#[test]
fn test_hasher_custom() {
    struct MyAllocator;
    impl Allocator for MyAllocator {
        fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let hasher = DefaultHashBuilder::default();
    let map: HashMap<String, String, DefaultHashBuilder, MyAllocator> = HashMap::with_hasher_in(hasher, MyAllocator);
    assert_eq!(map.hasher() as *const _, &hasher as *const _);
}

