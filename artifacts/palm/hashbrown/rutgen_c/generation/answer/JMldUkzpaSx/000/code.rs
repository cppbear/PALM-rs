// Answer 0

#[test]
fn test_entry_existing_key() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("a", 1);
    let entry = map.entry("a");
    
    match entry {
        Entry::Occupied(_) => {}
        _ => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_entry_vacant_key() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    let entry = map.entry("b");
    
    match entry {
        Entry::Vacant(_) => {}
        _ => panic!("Expected a vacant entry"),
    }
}

#[test]
fn test_entry_with_or_insert() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<char, i32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    
    for ch in "test".chars() {
        let counter = map.entry(ch).or_insert(0);
        *counter += 1;
    }
    
    assert_eq!(map.get(&'t'), Some(&2));
    assert_eq!(map.get(&'e'), Some(&1));
    assert_eq!(map.get(&'s'), Some(&1));
    assert_eq!(map.get(&'x'), None);
}

