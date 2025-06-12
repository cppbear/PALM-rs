// Answer 0

#[test]
fn test_index_occupied_entry() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries::<TestKey, TestValue>::new();
    let index = hashbrown::hash_table::OccupiedEntry::new(&mut entries, 0);
    let occupied_entry = OccupiedEntry::new(&mut entries, index);

    let entry = Entry::Occupied(occupied_entry);
    assert_eq!(entry.index(), 0);
}

#[test]
fn test_index_vacant_entry() {
    struct TestKey;
    struct TestValue;

    let map = RefMut::new();
    let hash_value = HashValue::default();
    let key = TestKey;
    let vacant_entry = VacantEntry {
        map,
        hash: hash_value,
        key,
    };

    let entry = Entry::Vacant(vacant_entry);
    assert_eq!(entry.index(), 0); // Assuming map.indices.len() returns 0 at initialization
}

