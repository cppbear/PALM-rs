// Answer 0

#[test]
fn test_iter_mut_with_multiple_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a HashTable and insert elements
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&1), 1, hasher_fn);
    table.insert_unique(hasher_fn(&2), 2, hasher_fn);
    table.insert_unique(hasher_fn(&3), 3, hasher_fn);

    // Update all values by doubling them
    for val in table.iter_mut() {
        *val *= 2;
    }

    assert_eq!(table.len(), 3);

    let mut vec: Vec<i32> = Vec::new();

    for val in &table {
        vec.push(*val);
    }

    // Sort and check the values
    vec.sort_unstable();
    assert_eq!(vec, [2, 4, 6]);
}

#[test]
fn test_iter_mut_empty_table() {
    use hashbrown::HashTable;

    // Create a HashTable with no elements
    let mut table: HashTable<i32, i32> = HashTable::new();

    assert_eq!(table.len(), 0);

    // Iterate over empty table should not panic
    for _val in table.iter_mut() {
        panic!("Should not be iterated over empty table");
    }

    // Check the length remains the same
    assert_eq!(table.len(), 0);
}

#[test]
fn test_iter_mut_after_inserting_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a HashTable and insert elements
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher_fn(&1), 1, hasher_fn);
    table.insert_unique(hasher_fn(&2), 2, hasher_fn);
    table.insert_unique(hasher_fn(&3), 3, hasher_fn);

    // Check iter_mut after inserting elements
    let mut iterate_values = 0;
    for val in table.iter_mut() {
        iterate_values += *val;
    }

    assert_eq!(iterate_values, 6);
}

