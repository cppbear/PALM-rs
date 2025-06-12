// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestKey;
    struct TestValue(i32);
    
    let mut entries = Entries::<TestKey, TestValue>::new();
    let key = TestKey;

    // Manually create an occupied entry for the test
    let occupied_entry = OccupiedEntry::new(&mut entries, hashbrown::hash_table::OccupiedEntry::new(0));
    entries.insert(key, TestValue(42)); // Assuming we have some insertion method

    let entry = Entry::Occupied(occupied_entry);

    // Call the function under test
    let result = entry.or_insert_with(|| TestValue(100));

    // Verify the result
    assert_eq!(result.get().0, 42); // Verify that it returns the existing value
    
    // Modify the existing value to observe changes
    result.get_mut().0 = 100;
    
    // Check the updated value
    assert_eq!(result.get().0, 100);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestKey;
    struct TestValue(i32);

    let mut entries = Entries::<TestKey, TestValue>::new();
    let key = TestKey;

    // Create a vacant entry manually for the test
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::default(), // Assuming default is valid here
        key,
    };
    
    let entry = Entry::Vacant(vacant_entry);

    // Call the function under test
    let result = entry.or_insert_with(|| TestValue(100));

    // Verify the result
    assert_eq!(result.get().0, 100); // Should insert the value
}

#[should_panic]
#[test]
fn test_or_insert_with_invalid_state() {
    struct TestKey;
    struct TestValue(i32);

    let mut entries = Entries::<TestKey, TestValue>::new();
    
    let key = TestKey;
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::default(),
        key,
    };
    
    // Creating an occupied entry whilst expecting a vacant state
    let entry = Entry::Occupied(OccupiedEntry::new(&mut entries, hashbrown::hash_table::OccupiedEntry::new(0)));

    // This should panic since we're trying to use it as if it's vacant.
    let _ = entry.or_insert_with(|| TestValue(100));
}

