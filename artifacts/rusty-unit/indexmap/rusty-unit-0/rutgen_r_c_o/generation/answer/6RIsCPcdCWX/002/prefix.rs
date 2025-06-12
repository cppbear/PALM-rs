// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestKey {
        id: usize,
    }

    struct TestValue {
        data: String,
    }

    let mut entries = Entries::<TestKey, TestValue>::new();
    let key = TestKey { id: 1 };
    let value = TestValue { data: "Test Value".to_string() };
    entries.insert(key.clone(), value);

    let occupied_entry = OccupiedEntry::new(&mut entries, entries.get_index(&key).unwrap());

    let entry = Entry::Occupied(occupied_entry);
    
    let result = entry.or_insert_with(|| TestValue { data: "Default Value".to_string() });
}

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestKey {
        id: usize,
    }

    struct TestValue {
        data: String,
    }

    let mut entries = Entries::<TestKey, TestValue>::new();
    let vacant_key = TestKey { id: 2 };

    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::default(),
        key: vacant_key.clone(),
    };

    let entry = Entry::Vacant(vacant_entry);
    
    let result = entry.or_insert_with(|| TestValue { data: "Inserted Value".to_string() });
}

