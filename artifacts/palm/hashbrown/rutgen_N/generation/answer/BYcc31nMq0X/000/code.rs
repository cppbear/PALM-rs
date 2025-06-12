// Answer 0

#[test]
fn test_remove_from_occupied_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // The table is empty
    assert!(table.is_empty() && table.capacity() == 0);

    table.insert_unique(hasher(&"poneyland"), "poneyland", hasher);
    let capacity_before_remove = table.capacity();

    if let Entry::Occupied(o) = table.entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher) {
        assert_eq!(o.remove().0, "poneyland");
    }

    assert!(table.find(hasher(&"poneyland"), |&x| x == "poneyland").is_none());
    // Now table holds no elements, but capacity is equal to the old one
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
    let hasher = |val: &_| hasher.hash_one(val);

    // Trying to remove an entry that does not exist
    let _ = table.entry(hasher(&"non_existent"), |&x| x == "non_existent", hasher);
}

