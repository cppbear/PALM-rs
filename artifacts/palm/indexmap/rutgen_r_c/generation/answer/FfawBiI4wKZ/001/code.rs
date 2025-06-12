// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    let hash_value = HashValue::default();
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: hash_value,
        key: TestKey,
    };

    let entry = Entry::Vacant(vacant_entry);
    let default_value = TestValue;

    let result = entry.or_insert(default_value);

    // Check that the result is a mutable reference to the inserted value
    assert!(result.is_some()); // We expect a value to be returned
}

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    entries.insert(TestKey, TestValue); // Preinsert to create an occupied entry

    let index = entries.get_index(&TestKey).unwrap();
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    let entry = Entry::Occupied(occupied_entry);
    let result = entry.or_insert(TestValue);

    // Check that the result is a mutable reference to the existing value
    assert!(result.is_some()); // We expect a value to be returned

    // Check that the previously occupied value is returned
    assert_eq!(result, &mut entries.get_mut(&TestKey).unwrap());
}

