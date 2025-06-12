// Answer 0

#[test]
fn test_insert_entry_occupied() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut entries = Entries::<TestKey, TestValue>::new();
    let key = TestKey(1);
    let value = TestValue(10);
    let occupied_entry = OccupiedEntry::new(&mut entries, /* some occupied entry index */);
    entries.insert(key.clone(), value.clone());

    let entry = Entry::Occupied(occupied_entry);
    let new_value = TestValue(20);
    let result = entry.insert_entry(new_value);

    assert_eq!(result.get(), &TestValue(20));
}

#[test]
fn test_insert_entry_vacant() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut entries = Entries::<TestKey, TestValue>::new();
    let key = TestKey(2);
    let value = TestValue(10);
    let vacant_entry = VacantEntry { map: RefMut::new(&mut entries), hash: HashValue::default(), key };

    let entry = Entry::Vacant(vacant_entry);
    let result = entry.insert_entry(value);

    assert_eq!(result.get(), &TestValue(10)); 
}

#[test]
fn test_insert_entry_empty_map() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut entries = Entries::<TestKey, TestValue>::new();
    let key = TestKey(3);
    let value = TestValue(10);
    let vacant_entry = VacantEntry { map: RefMut::new(&mut entries), hash: HashValue::default(), key };

    let entry = Entry::Vacant(vacant_entry);
    let result = entry.insert_entry(value);

    assert_eq!(result.get(), &TestValue(10)); 
}

