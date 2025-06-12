// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use indexmap::IndexMap;
    use indexmap::map::Entry;

    struct TestKey;
    struct TestValue {
        value: i32,
    }

    let mut map: IndexMap<TestKey, TestValue> = IndexMap::new();
    
    let entry = Entry::Vacant(map.vacant_entry().insert(TestKey));
    let result = entry.or_insert_with_key(|_key| TestValue { value: 42 });

    assert_eq!(result.value, 42);
}

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use indexmap::IndexMap;
    use indexmap::map::Entry;

    struct TestKey;
    struct TestValue {
        value: i32,
    }

    let mut map: IndexMap<TestKey, TestValue> = IndexMap::new();
    map.insert(TestKey, TestValue { value: 10 });

    let entry = Entry::Occupied(map.get_entry(&TestKey).unwrap());
    let result = entry.or_insert_with_key(|_key| TestValue { value: 42 });

    assert_eq!(result.value, 10);
}

#[test]
fn test_or_insert_with_key_multiple_insertions() {
    use indexmap::IndexMap;
    use indexmap::map::Entry;

    struct TestKey;
    struct TestValue {
        value: i32,
    }

    let mut map: IndexMap<TestKey, TestValue> = IndexMap::new();
    
    {
        let entry = Entry::Vacant(map.vacant_entry().insert(TestKey));
        let result = entry.or_insert_with_key(|_key| TestValue { value: 15 });
        assert_eq!(result.value, 15);
    }

    {
        let entry = Entry::Occupied(map.get_entry(&TestKey).unwrap());
        let result = entry.or_insert_with_key(|_key| TestValue { value: 25 });
        assert_eq!(result.value, 15);
    }
}

