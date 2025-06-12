// Answer 0

#[test]
fn test_retain_remove_all() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(DefaultHashBuilder::default());
    let hasher = |val: &_| val % 2; // Simple modulo hash for testing

    for x in 1..=6 {
        table.insert_unique(hasher(&x) as u64, x, hasher);
    }

    // Retain only even numbers
    table.retain(|&mut x| x % 2 == 0);

    // The table should now contain only even numbers: 2, 4, 6
    assert_eq!(table.len(), 3);
}

#[test]
fn test_retain_no_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(DefaultHashBuilder::default());
    let hasher = |val: &_| val % 2; // Simple modulo hash for testing

    // The table is empty, so retain should not alter its state
    table.retain(|_| false);

    assert_eq!(table.len(), 0);
}

#[test]
fn test_retain_all_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new_in(DefaultHashBuilder::default());
    let hasher = |val: &_| val % 2; // Simple modulo hash for testing

    for x in 1..=5 {
        table.insert_unique(hasher(&x) as u64, x, hasher);
    }

    // Retain all elements
    table.retain(|_| true);

    // The table should still contain all original elements: 1, 2, 3, 4, 5
    assert_eq!(table.len(), 5);
}

