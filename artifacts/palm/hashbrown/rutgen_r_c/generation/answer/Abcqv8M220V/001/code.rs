// Answer 0

#[test]
fn test_key_retrieval() {
    use crate::raw::RawTable;
    use crate::DefaultHashBuilder;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    map.table.insert(("example_key", 42));

    let entry = OccupiedEntry {
        hash: 0,
        elem: Bucket {
            ptr: core::ptr::NonNull::from(&("example_key", 42)),
        },
        table: &mut map,
    };

    assert_eq!(entry.key(), &"example_key");
}

#[test]
#[should_panic]
fn test_key_on_empty_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    let map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let entry = OccupiedEntry {
        hash: 0,
        elem: Bucket {
            ptr: core::ptr::NonNull::dangling(),
        },
        table: &mut map,
    };

    let _ = entry.key(); // This should panic since the entry is empty
}

