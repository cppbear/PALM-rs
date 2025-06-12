// Answer 0

#[test]
fn test_entry_occupied() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &(u64, &str)| hasher.hash_one(val);

    table.insert_unique(hasher(&(1, "a")), (1, "a"), hasher);
    let entry = table.entry(hasher(&(1, "a")), |val| val.0 == 1, hasher);

    match entry {
        Entry::Occupied(_) => {}
        _ => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_entry_vacant() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &(u64, &str)| hasher.hash_one(val);

    let entry = table.entry(hasher(&(2, "b")), |val| val.0 == 2, hasher);
    
    match entry {
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert((2, "b"));
        }
        _ => panic!("Expected a vacant entry"),
    }

    assert_eq!(table.find(hasher(&(2, "b")), |val| val.0 == 2), Some(&(2, "b")));
}

#[test]
fn test_entry_insert_and_remove() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &(u64, &str)| hasher.hash_one(val);
    
    table.insert_unique(hasher(&(1, "a")), (1, "a"), hasher);
    
    if let Entry::Occupied(entry) = table.entry(hasher(&(1, "a")), |val| val.0 == 1, hasher) {
        entry.remove();
    }
    
    assert_eq!(table.find(hasher(&(1, "a")), |val| val.0 == 1), None);
}

