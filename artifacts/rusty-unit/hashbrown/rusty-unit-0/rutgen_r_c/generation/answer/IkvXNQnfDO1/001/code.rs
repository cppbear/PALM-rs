// Answer 0

#[test]
fn test_remove_entry() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    // Insert an element into the HashMap
    map.table.insert(Bucket { ptr: NonNull::new(Box::into_raw(Box::new(("poneyland", 12)))) });
    
    if let OccupiedEntry { table: &mut map, elem, .. } = map.get_occupied_entry("poneyland").unwrap() {
        assert_eq!(OccupiedEntry { table: &mut map, elem, .. }.remove(), 12);
    }
    
    assert_eq!(map.table.len(), 0);
}

