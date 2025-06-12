// Answer 0

#[test]
fn test_key_function() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    map.entry("poneyland").or_insert(12);

    match map.entry("poneyland") {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            assert_eq!(entry.key(), &"poneyland");
        },
    }
}

