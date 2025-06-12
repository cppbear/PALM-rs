// Answer 0

#[test]
fn test_replace_entry_with_updates_value() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, _> = [("a", 100), ("b", 200)].into();
    let hash_builder = make_hasher();
    
    let raw_entry = match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 100);
            Some(v + 900)
        }),
    };

    let raw_entry = match raw_entry {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"a");
            assert_eq!(v, 1000);
            None
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&"a"), None);
}

#[test]
fn test_replace_entry_with_removes_entry() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, _> = [("x", 500)].into();
    let hash_builder = make_hasher();
    
    let raw_entry = match map.raw_entry_mut().from_key(&"x") {
        RawEntryMut::Vacant(_) => panic!(),
        RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {
            assert_eq!(k, &"x");
            assert_eq!(v, 500);
            None
        }),
    };

    match raw_entry {
        RawEntryMut::Vacant(_) => { },
        RawEntryMut::Occupied(_) => panic!(),
    };

    assert_eq!(map.get(&"x"), None);
}

