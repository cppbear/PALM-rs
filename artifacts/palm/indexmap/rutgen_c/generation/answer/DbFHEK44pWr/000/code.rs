// Answer 0

#[test]
fn test_or_insert_with_key_occupied_entry() {
    struct TestKey;
    struct TestValue(u32);
    
    let mut entries = vec![
        (TestKey, TestValue(42)), // existing entry
    ];
    
    let occupied_entry = {
        let index = 0; // index of occupied entry
        let occupied_entry = hashbrown::hash_table::OccupiedEntry::from_raw_index(&mut entries, index);
        OccupiedEntry::new(&mut entries, occupied_entry)
    };

    let entry = Entry::Occupied(occupied_entry);
    let result = entry.or_insert_with_key(|_key| TestValue(100));

    assert_eq!(result.0, 42);
}

#[test]
fn test_or_insert_with_key_vacant_entry() {
    struct TestKey;
    struct TestValue(u32);

    let key = TestKey;
    let mut entries: Vec<(TestKey, TestValue)> = vec![];

    let map_ref = RefMut::new(&mut entries);
    let vacant_entry = VacantEntry {
        map: map_ref,
        hash: HashValue::default(),
        key,
    };

    let entry = Entry::Vacant(vacant_entry);
    let result = entry.or_insert_with_key(|_key| TestValue(100));

    assert_eq!(result.0, 100);
}

#[test]
#[should_panic]
fn test_or_insert_with_key_on_empty_map() {
    struct TestKey;
    struct TestValue(u32);

    let key = TestKey;
    let mut entries: Vec<(TestKey, TestValue)> = vec![];

    let map_ref = RefMut::new(&mut entries);
    let vacant_entry = VacantEntry {
        map: map_ref,
        hash: HashValue::default(),
        key,
    };

    let entry = Entry::Vacant(vacant_entry);
    let _ = entry.or_insert_with_key(|_key| TestValue(100));
}

