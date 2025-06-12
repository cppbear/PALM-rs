// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // First, insert a value into the HashTable
    let entry = table
        .entry(hasher(&"test_key"), |&x| x == "test_key", hasher)
        .insert("initial_value");

    // Now replace the existing value with a new one
    let occupied_entry = table
        .entry(hasher(&"test_key"), |&x| x == "test_key", hasher)
        .insert("new_value");

    assert_eq!(occupied_entry.get(), &"new_value");
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Trying to insert a value in a vacant entry
    let entry = table
        .entry(hasher(&"another_key"), |&x| x == "another_key", hasher)
        .insert("first_value");

    assert_eq!(entry.get(), &"first_value");
}

