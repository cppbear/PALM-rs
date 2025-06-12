// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();
    map.insert("key1", 42);

    let entry = Entry::Occupied(OccupiedEntry {
        hash: 0,
        elem: Bucket::new((Box::from("key1"), 42)),
        table: &mut map,
    });

    let value: &mut i32 = entry.or_insert(10);
    assert_eq!(*value, 42); // Ensure that the reference points to the existing value
}

#[test]
fn test_or_insert_with_vacant_entry() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new();

    let entry = Entry::Vacant(VacantEntry {
        hash: 0,
        key: Box::from("new_key"),
        table: &mut map,
    });

    let value: &mut i32 = entry.or_insert(100);
    assert_eq!(*value, 100); // Ensure that a new value is inserted
    assert_eq!(map.get("new_key"), Some(&100));
}

