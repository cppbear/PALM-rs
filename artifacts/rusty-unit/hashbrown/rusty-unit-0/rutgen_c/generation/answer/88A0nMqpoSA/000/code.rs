// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Assert inserting a new vacant entry
    let entry = table
        .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher);
    let occupied_entry = entry.or_insert("poneyland");
    assert!(table
        .find(hasher(&"poneyland"), |&x| x == "poneyland")
        .is_some());
    assert_eq!(occupied_entry.elem, table.find(hasher(&"poneyland"), |&x| x == "poneyland").unwrap());

    // Check that table still has only one entry
    assert_eq!(table.len(), 1);
}

#[test]
fn test_or_insert_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert initial value
    table
        .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher)
        .or_insert("poneyland");

    // Try inserting a duplicate value
    let entry = table
        .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher);
    let occupied_entry = entry.or_insert("poneyland");
    
    // Check that the value wasn't inserted again
    assert_eq!(table.len(), 1);
    assert!(table
        .find(hasher(&"poneyland"), |&x| x == "poneyland")
        .is_some());
    assert_eq!(occupied_entry.elem, table.find(hasher(&"poneyland"), |&x| x == "poneyland").unwrap());
}

