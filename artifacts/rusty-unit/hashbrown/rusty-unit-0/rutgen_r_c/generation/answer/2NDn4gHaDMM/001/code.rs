// Answer 0

#[test]
fn test_insert_on_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    
    let entry = map.entry("test_key");
    
    assert!(matches!(entry, Entry::Vacant(_)));

    let occupied_entry = entry.insert(42);
    
    assert_eq!(occupied_entry.key(), &"test_key");
}

#[test]
fn test_insert_on_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::Entry;

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("existing_key", 100);

    let entry = map.entry("existing_key");
    
    assert!(matches!(entry, Entry::Occupied(_)));

    let occupied_entry_before = entry.insert(50);
    
    assert_eq!(occupied_entry_before.get(), &50);
    assert_eq!(occupied_entry_before.key(), &"existing_key");
}

