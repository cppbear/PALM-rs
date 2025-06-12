// Answer 0

#[test]
fn test_insert_overwrites_value() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unreachable!()
        }

        unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {
            unreachable!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    map.insert("poneyland", 12);

    let mut occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket {
            ptr: core::ptr::NonNull::new(&mut 12).unwrap(),
        },
        table: &mut map,
    };
    
    let old_value = occupied_entry.insert(15);
    assert_eq!(old_value, 12);
}

#[test]
fn test_insert_same_value() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unreachable!()
        }

        unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {
            unreachable!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    map.insert("poneyland", 12);

    let mut occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket {
            ptr: core::ptr::NonNull::new(&mut 12).unwrap(),
        },
        table: &mut map,
    };

    let old_value = occupied_entry.insert(12);
    assert_eq!(old_value, 12);
}

#[test]
fn test_insert_updates_value() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unreachable!()
        }

        unsafe fn deallocate(&self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {
            unreachable!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    map.insert("poneyland", 12);

    let mut occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket {
            ptr: core::ptr::NonNull::new(&mut 12).unwrap(),
        },
        table: &mut map,
    };

    assert_eq!(occupied_entry.insert(20), 12);
    assert_eq!(*map.get("poneyland").unwrap(), 20);
}

