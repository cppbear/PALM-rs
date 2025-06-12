// Answer 0

#[test]
fn test_drain_basic_functionality() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), TestAllocator);
    map.insert(1, "a");
    map.insert(2, "b");
    let capacity_before_drain = map.capacity();

    let mut drained = map.drain();

    assert_eq!(drained.next(), Some((1, "a")));
    assert_eq!(drained.next(), Some((2, "b")));
    assert_eq!(drained.next(), None);

    assert!(map.is_empty());
    assert_eq!(map.capacity(), capacity_before_drain);
}

#[test]
fn test_drain_with_drop() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), TestAllocator);
    map.insert(1, "a");
    map.insert(2, "b");

    {
        let _drain = map.drain();
    }

    assert!(map.is_empty());
}

#[test]
fn test_drain_no_elements() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new(1 as *mut u8).unwrap())
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap::with_hasher_in(DefaultHashBuilder::new(), TestAllocator);
    
    let drained = map.drain();

    assert_eq!(drained.next(), None);
    assert!(map.is_empty());
}

