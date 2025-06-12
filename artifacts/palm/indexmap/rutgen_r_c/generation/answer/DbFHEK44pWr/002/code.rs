// Answer 0

#[test]
fn test_or_insert_with_key_occupied() {
    struct Key;
    struct Value(i32);
    
    let mut entries = hashbrown::HashMap::new();
    entries.insert(0, Value(42));
    
    let occupied_entry_key = 0;
    let occupied_entry = Entries {
        entries: &mut entries,
        index: occupied_entry_key,
    };

    let entry = Entry::Occupied(OccupiedEntry::new(&mut occupied_entry, occupied_entry_key));

    let result = entry.or_insert_with_key(|key| Value(*key as i32 + 10));

    assert_eq!(result.get_mut(), &mut Value(42));
}

#[test]
fn test_or_insert_with_key_vacant() {
    struct Key;
    struct Value(i32);
    
    let mut entries = hashbrown::HashMap::new();
    let vacant_key = 1; // Assume this key is not present in the map
    
    let vacant_entry = VacantEntry {
        map: RefMut::new(&mut entries),
        hash: HashValue::from(vacant_key),
        key: vacant_key,
    };

    let entry = Entry::Vacant(vacant_entry);

    let result = entry.or_insert_with_key(|key| Value(*key as i32 + 10));

    assert_eq!(result.get_mut(), &mut Value(11));
    assert!(entries.contains_key(&vacant_key));
}

