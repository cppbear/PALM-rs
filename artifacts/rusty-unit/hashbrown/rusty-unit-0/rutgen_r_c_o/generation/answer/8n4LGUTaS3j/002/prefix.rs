// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"item1"), ("item1", 10), hasher);
    
    let entry = table.entry(
        hasher(&"item1"),
        |&(x, _)| x == "item1",
        |(k, _)| hasher(&k),
    );

    entry.and_modify(|(_, v)| *v += 5);
}

#[test]
fn test_and_modify_multiple_times() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"item2"), ("item2", 20), hasher);
    
    let entry = table.entry(
        hasher(&"item2"),
        |&(x, _)| x == "item2",
        |(k, _)| hasher(&k),
    );

    entry.and_modify(|(_, v)| *v += 10);
    entry.and_modify(|(_, v)| *v += 1);
}

#[test]
fn test_and_modify_with_string_key() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(String, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"item3".to_string()), ("item3".to_string(), 30), hasher);
    
    let entry = table.entry(
        hasher(&"item3".to_string()),
        |&(ref x, _)| x == "item3",
        |(k, _)| hasher(&k),
    );

    entry.and_modify(|(_, v)| *v += 15);
}

#[test]
fn test_and_modify_edge_case() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, usize)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"item4"), ("item4", 0), hasher);
    
    let entry = table.entry(
        hasher(&"item4"),
        |&(x, _)| x == "item4",
        |(k, _)| hasher(&k),
    );

    entry.and_modify(|(_, v)| *v = usize::MAX);
}

