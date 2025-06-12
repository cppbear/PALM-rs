// Answer 0

#[test]
fn test_key() {
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
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    let key = "poneyland";
    let vacant_entry = VacantEntry {
        hash: 0,
        key,
        table: &mut map,
    };

    assert_eq!(vacant_entry.key(), &"poneyland");
}

