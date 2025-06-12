// Answer 0

#[test]
fn test_into_mut_existing_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

    assert_eq!(
        table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),
        Some(&("poneyland", 12))
    );

    let value: &mut (&str, u32);
    match table.entry(
        hasher(&"poneyland"),
        |&(x, _)| x == "poneyland",
        |(k, _)| hasher(&k),
    ) {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
    value.1 += 10;

    assert_eq!(
        table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),
        Some(&("poneyland", 22))
    );
}

#[test]
#[should_panic]
fn test_into_mut_vacant_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    let value: &mut (&str, u32);
    match table.entry(
        hasher(&"nonexistent"),
        |&(x, _)| x == "nonexistent",
        |(k, _)| hasher(&k),
    ) {
        Entry::Occupied(_) => value = panic!("This should panic!"),
        Entry::Vacant(_) => panic!(),
    }
}

