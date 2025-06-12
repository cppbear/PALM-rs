// Answer 0

#[test]
fn test_remove_entry() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, DummyAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::default(),
        table: RawTable::new(),
    };

    assert!(map.table.is_empty());

    map.entry("poneyland").or_insert(12);

    if let OccupiedEntry { key, value, .. } = map.entry("poneyland") {
        assert_eq!(value.remove(), 12);
    }

    assert_eq!(map.table.contains_key("poneyland"), false);
    assert!(map.table.is_empty());
}

