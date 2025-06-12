// Answer 0

#[test]
fn test_remove_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Initially empty table
    assert!(table.is_empty() && table.capacity() == 0);

    // Inserting an element into the table
    table.insert_unique(hasher_fn(&"poneyland"), "poneyland", hasher_fn);
    let capacity_before_remove = table.capacity();

    // Removing the element and asserting the return value
    if let Entry::Occupied(o) = table.entry(hasher_fn(&"poneyland"), |&x| x == "poneyland", hasher_fn) {
        let (val, vacant_entry) = o.remove();
        assert_eq!(val, "poneyland");
        // Validate VacantEntry properties
        assert_eq!(vacant_entry.hash, hasher_fn(&"poneyland"));
        assert_eq!(vacant_entry.table.len(), 0);
        assert_eq!(vacant_entry.table.capacity(), capacity_before_remove);
    }

    // Ensure element was removed
    assert!(table.find(hasher_fn(&"poneyland"), |&x| x == "poneyland").is_none());
    // Check that capacity remains the same
    assert!(table.len() == 0 && table.capacity() == capacity_before_remove);
}

#[test]
#[should_panic]
fn test_remove_non_existent_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Trying to remove an element that does not exist should panic
    if let Entry::Occupied(o) = table.entry(hasher_fn(&"nonexistent"), |&x| x == "nonexistent", hasher_fn) {
        o.remove();
    }
}

