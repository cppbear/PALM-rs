// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"test_key"), "initial_value", hasher);

    let entry = table
        .entry(hasher(&"test_key"), |&x| x == "test_key", hasher)
        .insert("updated_value");

    assert_eq!(entry.get(), &"updated_value");
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let entry = table
        .entry(hasher(&"new_key"), |&x| x == "new_key", hasher)
        .insert("new_value");

    assert_eq!(entry.get(), &"new_value");
}

#[test]
fn test_insert_replace_value() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"replace_key"), "old_value", hasher);

    let entry = table
        .entry(hasher(&"replace_key"), |&x| x == "replace_key", hasher)
        .insert("new_value");

    assert_eq!(entry.get(), &"new_value");
}

