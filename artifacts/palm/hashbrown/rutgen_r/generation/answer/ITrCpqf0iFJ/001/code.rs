// Answer 0

#[test]
fn test_iter_hash_mut_with_existing_hash() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&1), 3, hasher);
    table.insert_unique(hasher(&2), 5, hasher);

    let mut iter = table.iter_hash_mut(hasher(&1));

    for val in &mut iter {
        *val *= 2;
    }

    assert_eq!(table.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in &table {
        vec.push(*val);
    }

    assert!(vec.contains(&4));
    assert!(vec.contains(&6));
}

#[test]
fn test_iter_hash_mut_with_nonexistent_hash() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 10, hasher);
    table.insert_unique(hasher(&2), 20, hasher);

    let mut iter = table.iter_hash_mut(hasher(&3));

    let mut count = 0;
    for _ in iter {
        count += 1;
    }

    assert_eq!(count, 0);  // no values should be returned for a non-existent hash
}

#[should_panic]
fn test_iter_hash_mut_panic_on_invalid_access() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&1), 1, hasher);

    let mut iter = table.iter_hash_mut(hasher(&1));

    // Simulating invalid access by dropping the table, which should panic
    drop(table);

    for _ in iter {
        // This should panic
        panic!("Trying to access an invalid iterator after the original table was dropped.");
    }
}

