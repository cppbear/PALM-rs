// Answer 0

#[test]
fn test_or_insert_with_key_vacant_entry() {
    struct DummyKey;
    struct DummyValue;

    let key = DummyKey;
    let mut map = {
        let mut entries = Vec::new();
        entries.push((key, DummyValue));
        entries
    };

    let vacant_entry = Entry::Vacant(VacantEntry {
        map: RefMut::new(map),
        hash: HashValue::default(),
        key,
    });

    let result = vacant_entry.or_insert_with_key(|k| DummyValue);
    
    // Verify the result is a reference to the newly inserted value
    assert!(result.is_some());
}

#[test]
#[should_panic]
fn test_or_insert_with_key_panic_on_occupied_entry() {
    struct DummyKey;
    struct DummyValue;

    let key = DummyKey;
    let mut entries = vec![(key, DummyValue)];
    
    let occupied_entry = Entry::Occupied(OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0)));

    // This call should panic as the entry is occupied
    let _ = occupied_entry.or_insert_with_key(|_k| DummyValue);
}

