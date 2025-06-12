// Answer 0

#[test]
fn test_or_insert_nonexistent_key() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // nonexistent key
    table
        .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher)
        .or_insert("poneyland");
    assert!(table
        .find(hasher(&"poneyland"), |&x| x == "poneyland")
        .is_some());
}

#[test]
fn test_or_insert_existing_key() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert and check existing key
    table
        .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher)
        .or_insert("poneyland");
    table
        .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher)
        .or_insert("poneyland");
    assert!(table
        .find(hasher(&"poneyland"), |&x| x == "poneyland")
        .is_some());
    assert_eq!(table.len(), 1);
}

