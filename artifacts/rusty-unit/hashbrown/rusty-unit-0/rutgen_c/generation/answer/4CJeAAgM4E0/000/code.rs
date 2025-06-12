// Answer 0

#[test]
fn test_is_empty_initially_empty() {
    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::from(&mut 0))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let set: HashSet<i32, DefaultHashBuilder, MyAllocator> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    assert!(set.is_empty());
}

#[test]
fn test_is_empty_after_inserting_element() {
    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::from(&mut 0))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut set: HashSet<i32, DefaultHashBuilder, MyAllocator> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    set.map.insert(1, ());
    assert!(!set.is_empty());
}

#[test]
fn test_is_empty_after_clear() {
    struct MyAllocator;

    impl Allocator for MyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::from(&mut 0))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let mut set: HashSet<i32, DefaultHashBuilder, MyAllocator> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder, table: RawTable::new() } };
    set.map.insert(1, ());
    set.clear();
    assert!(set.is_empty());
}

