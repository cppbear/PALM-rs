// Answer 0

#[test]
fn test_capacity_after_initialization() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let allocator = TestAllocator;
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(100, DefaultHashBuilder::new(), allocator);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 100);
}

#[test]
fn test_capacity_zero() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
            unimplemented!()
        }
    }

    let allocator = TestAllocator;
    let map: HashMap<i32, i32, DefaultHashBuilder, TestAllocator> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), allocator);
    assert_eq!(map.len(), 0);
    assert!(map.capacity() >= 0);
}

