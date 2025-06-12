// Answer 0

#[test]
fn test_into_mut_valid() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&"apple"), ("apple", 10), |(k, _)| hasher(&k));
    
    let value: &mut (&str, u32);
    match table.entry(
        hasher(&"apple"),
        |&(x, _)| x == "apple",
        |(k, _)| hasher(&k),
    ) {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
    value.1 += 10;
}

#[test]
fn test_into_mut_multiple_entries() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    table.insert_unique(hasher(&"banana"), ("banana", 5), |(k, _)| hasher(&k));
    table.insert_unique(hasher(&"cherry"), ("cherry", 8), |(k, _)| hasher(&k));

    let value: &mut (&str, u32);
    match table.entry(
        hasher(&"banana"),
        |&(x, _)| x == "banana",
        |(k, _)| hasher(&k),
    ) {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
    value.1 += 20;

    match table.entry(
        hasher(&"cherry"),
        |&(x, _)| x == "cherry",
        |(k, _)| hasher(&k),
    ) {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
    value.1 += 15;
}

#[test]
#[should_panic]
fn test_into_mut_panic_on_vacant_entry() {
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
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_into_mut_large_entry() {
    use hashbrown::hash_table::Entry;
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(u32, String)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    let large_string = "x".repeat(10000);
    table.insert_unique(1, (1, large_string.clone()), |(k, _)| hasher(&k));

    let value: &mut (u32, String);
    match table.entry(
        hasher(&1),
        |&(x, _)| x == 1,
        |(k, _)| hasher(&k),
    ) {
        Entry::Occupied(entry) => value = entry.into_mut(),
        Entry::Vacant(_) => panic!(),
    }
    value.1.push_str(" updated");
}

