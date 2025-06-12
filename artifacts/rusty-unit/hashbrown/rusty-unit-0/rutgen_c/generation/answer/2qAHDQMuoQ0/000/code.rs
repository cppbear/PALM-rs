// Answer 0

#[test]
fn test_into_mut() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    
    map.insert("poneyland", 12);

    let mut value: &mut u32;
    match map.entry("poneyland") {
        Entry::Occupied(entry) => {
            value = entry.into_mut();
        },
        Entry::Vacant(_) => panic!(),
    }

    *value += 10;

    assert_eq!(map["poneyland"], 22);
}

#[test]
#[should_panic]
fn test_into_mut_non_existent_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let _value: &mut u32;
    match map.entry("non_existent") {
        Entry::Occupied(entry) => {
            _value = entry.into_mut();
        },
        Entry::Vacant(_) => panic!(),
    }
}

