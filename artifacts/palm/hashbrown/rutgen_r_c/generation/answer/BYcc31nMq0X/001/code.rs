// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    assert!(table.is_empty() && table.capacity() == 0);

    table.insert_unique(hasher_fn(&"test_key"), "test_value", hasher_fn);
    let capacity_before_remove = table.capacity();

    if let Some(o) = table.find(hasher_fn(&"test_key"), |&x| x == "test_value") {
        let occupied_entry = OccupiedEntry {
            hash: hasher_fn(&"test_key"),
            bucket: o,
            table: &mut table,
        };
        
        let (val, vacant_entry) = occupied_entry.remove();
        assert_eq!(val, "test_value");
        assert_eq!(vacant_entry.hash, occupied_entry.hash);
        assert!(table.find(hasher_fn(&"test_key"), |&x| x == "test_value").is_none());
        assert!(table.len() == 0 && table.capacity() == capacity_before_remove);
    } else {
        panic!("Failed to find the occupied entry.");
    }
}

#[test]
#[should_panic]
fn test_remove_empty_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    // Attempt to remove from an empty table should panic
    let result = table.find(hasher_fn(&"non_existent_key"), |&x| x == "non_existent_value");
    if let Some(o) = result {
        let occupied_entry = OccupiedEntry {
            hash: hasher_fn(&"non_existent_key"),
            bucket: o,
            table: &mut table,
        };
        occupied_entry.remove();
    } else {
        panic!("No occupied entry found for removal.");
    }
}

