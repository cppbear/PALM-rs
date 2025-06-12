// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<&str> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hash_fn = |val: &_| hasher.hash_one(val);

    // Inserting a new value
    if let Entry::Vacant(o) = table.entry(hash_fn(&"poneyland"), |&x| x == "poneyland", hash_fn) {
        o.insert("poneyland");
    }

    // Verifying the value was inserted
    assert_eq!(
        table.find(hash_fn(&"poneyland"), |&x| x == "poneyland"),
        Some(&"poneyland")
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
    let hash_fn = |val: &_| hasher.hash_one(val);

    // Inserting a first value
    if let Entry::Vacant(o) = table.entry(hash_fn(&"poneyland"), |&x| x == "poneyland", hash_fn) {
        o.insert("poneyland");
    }

    // Attempting to insert a duplicate value, this should panic
    if let Entry::Occupied(_) = table.entry(hash_fn(&"poneyland"), |&x| x == "poneyland", hash_fn) {
        panic!("Duplicate entry inserted!");
    }
}

