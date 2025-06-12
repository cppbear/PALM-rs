// Answer 0

#[test]
fn test_into_mut_valid_case() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Insert an entry into the hash table
    table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

    let mut value: &mut (&str, u32);
    match table.entry(
        hasher(&"poneyland"),
        |&(x, _)| x == "poneyland",
        |(k, _)| hasher(&k),
    ) {
        hashbrown::hash_table::Entry::Occupied(entry) => {
            value = entry.into_mut(); // Valid case
        },
        hashbrown::hash_table::Entry::Vacant(_) => panic!(),
    }
    
    value.1 += 10;

    assert_eq!(
        table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),
        Some(&("poneyland", 22))
    );
}

#[test]
#[should_panic]
fn test_into_mut_on_empty_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    // Try to access an entry that does not exist
    match table.entry(
        hasher(&"non_existent"),
        |&(x, _)| x == "non_existent",
        |(k, _)| hasher(&k),
    ) {
        hashbrown::hash_table::Entry::Occupied(_) => panic!(),
        hashbrown::hash_table::Entry::Vacant(_) => {
            // This should not panic
        },
    }
}

