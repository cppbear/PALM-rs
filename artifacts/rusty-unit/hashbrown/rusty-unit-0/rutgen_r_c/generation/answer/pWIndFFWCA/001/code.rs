// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::HashMap;

    // Create a HashMap for testing
    let mut map: HashMap<String, u32> = HashMap::new();

    // Define a structure to create a VacantEntryRef directly
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let key = "test_key";
    let hash = 0; // Dummy hash
    let value = 5;

    // Create a VacantEntryRef directly
    let vacant_entry = VacantEntryRef {
        hash,
        key,
        table: &mut map,
    };

    // Call or_insert on the VacantEntryRef and capture the mutable reference
    let insert_result = vacant_entry.insert(value);

    // Verify the inserted value
    assert_eq!(*insert_result, 5);
    assert_eq!(map[key], 5);
}

#[test]
fn test_or_insert_occupied_entry() {
    use hashbrown::HashMap;

    // Create a HashMap and insert an initial value
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_owned(), 3);

    // Create a structure for the occupied entry
    struct TestOccupiedEntry<'a> {
        value: &'a mut u32,
    }

    // Mimicking the entry being occupied
    let occupied_entry = TestOccupiedEntry {
        value: map.get_mut("poneyland").unwrap(),
    };

    // Use EntryRef::Occupied to test the or_insert method
    let entry_ref = EntryRef::Occupied(occupied_entry);
    let new_value = *entry_ref.or_insert(10); // Should not insert as it is occupied

    // Verify that or_insert does not change the value since the entry is occupied
    assert_eq!(new_value, 3);
    *entry_ref.into_mut() *= 2; // This should update the existing value

    assert_eq!(map["poneyland"], 6); // Verify updated value 
}

