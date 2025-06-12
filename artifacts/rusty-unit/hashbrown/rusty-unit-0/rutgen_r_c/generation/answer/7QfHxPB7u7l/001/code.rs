// Answer 0

#[test]
fn test_entry_ref_key_vacant() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    let key_ref = "test_key";

    let vacant_entry_ref = EntryRef::Vacant(VacantEntryRef {
        hash: 0,
        key: &key_ref,
        table: &mut map,
    });

    assert_eq!(vacant_entry_ref.key(), "test_key");
}

#[test]
fn test_entry_ref_key_occupied() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<String, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    map.insert("occupied_key".to_string(), 10);
    
    let occupied_entry_ref = EntryRef::Occupied(OccupiedEntry {
        hash: 0,
        bucket: Bucket::new(("occupied_key".to_string(), 10)),
        table: &mut map,
    });

    assert_eq!(occupied_entry_ref.key(), &"occupied_key");
}

