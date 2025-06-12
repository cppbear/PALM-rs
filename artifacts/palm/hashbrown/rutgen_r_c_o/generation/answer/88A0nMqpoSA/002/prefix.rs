// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"key1"), "value1", hasher);
    let entry = table.entry(hasher(&"key1"), |&x| x == "key1", hasher);
    entry.or_insert("new_value1");

    let entry2 = table.entry(hasher(&"key1"), |&x| x == "key1", hasher);
    entry2.or_insert("new_value1");
}

#[test]
fn test_or_insert_with_existing_key() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"key2"), "value2", hasher);
    let entry = table.entry(hasher(&"key2"), |&x| x == "key2", hasher);
    entry.or_insert("new_value2");

    let entry2 = table.entry(hasher(&"key2"), |&x| x == "key2", hasher);
    entry2.or_insert("another_value2");
}

#[test]
fn test_or_insert_with_multiple_occurrences() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"key3"), "value3", hasher);
    for _ in 0..10 {
        let entry = table.entry(hasher(&"key3"), |&x| x == "key3", hasher);
        entry.or_insert("repeated_value3");
    }
}

#[test]
fn test_or_insert_with_various_types() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 10, hasher);
    let entry = table.entry(hasher(&1), |&x| x == &1, hasher);
    entry.or_insert(20);

    let entry2 = table.entry(hasher(&1), |&x| x == &1, hasher);
    entry2.or_insert(30);
}

