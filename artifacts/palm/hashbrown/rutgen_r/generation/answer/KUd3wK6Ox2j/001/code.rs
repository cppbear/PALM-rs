// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    if let Entry::Vacant(o) = table.entry(hasher(&"unique_key"), |&x| x == "unique_key", hasher) {
        o.insert("unique_value");
    }
    assert_eq!(
        table.find(hasher(&"unique_key"), |&x| x == "unique_key"),
        Some(&"unique_value")
    );
}

#[test]
#[should_panic]
fn test_insert_duplicate_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    if let Entry::Vacant(o) = table.entry(hasher(&"duplicate_key"), |&x| x == "duplicate_key", hasher) {
        o.insert("duplicate_value");
    }

    // This should panic because we are attempting to insert a second value for the same key
    if let Entry::Occupied(mut o) = table.entry(hasher(&"duplicate_key"), |&x| x == "duplicate_key", hasher) {
        o.insert("another_value");
    }
}

