// Answer 0

#[test]
fn test_entry_occupied_get() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&"test_key"), "test_value", hasher);

    match table.entry(hasher(&"test_key"), |&x| x == "test_value", hasher) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            let value: &str = entry.get();
            assert_eq!(value, &"test_value");
        },
    }
}

#[test]
#[should_panic]
fn test_entry_vacant_get() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    match table.entry(hasher(&"non_existent_key"), |&x| x == "non_existent_value", hasher) {
        Entry::Vacant(_) => {
            let _: &str = unreachable!("Expected entry to be occupied");
        },
        Entry::Occupied(entry) => {
            let _value: &str = entry.get();
        },
    }
}

