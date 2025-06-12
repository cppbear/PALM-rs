// Answer 0

#[test]
fn test_shrink_to_fit() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
}

#[test]
fn test_shrink_to_fit_empty_map() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity(10);
    assert!(map.capacity() >= 10);
    map.shrink_to_fit();
    assert!(map.capacity() == 0);
}

