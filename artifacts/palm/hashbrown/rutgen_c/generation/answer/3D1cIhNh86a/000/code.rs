// Answer 0

#[test]
fn test_get_mut() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    map.insert("poneyland", 12);

    if let Some(mut entry) = map.get_mut("poneyland") {
        *entry.get_mut() += 10;
        assert_eq!(*entry.get(), 22);
        *entry.get_mut() += 2;
    }

    assert_eq!(map.get("poneyland").unwrap(), &24);
}

