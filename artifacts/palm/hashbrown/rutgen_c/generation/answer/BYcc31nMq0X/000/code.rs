// Answer 0

#[test]
fn test_remove_function() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    // The table is empty
    assert!(table.is_empty() && table.capacity() == 0);

    // Inserting a value into the HashTable
    table.insert_unique(hasher(&"poneyland"), "poneyland", hasher);
    let capacity_before_remove = table.capacity();

    // Accessing the occupied entry
    if let Entry::Occupied(o) = table.entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher) {
        assert_eq!(o.remove().0, "poneyland");
    }

    // Checking if the entry was removed
    assert!(table
        .find(hasher(&"poneyland"), |&x| x == "poneyland")
        .is_none());
    
    // Ensuring the table is now empty but capacity is unchanged
    assert!(table.len() == 0 && table.capacity() == capacity_before_remove);
}

