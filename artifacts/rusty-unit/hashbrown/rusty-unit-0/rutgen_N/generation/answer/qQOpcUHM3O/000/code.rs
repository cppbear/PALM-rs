// Answer 0

#[test]
fn test_retain_with_even_numbers() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x % 2 == 0);
    assert_eq!(table.len(), 3);
}

#[test]
fn test_retain_with_all_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut _| true);
    assert_eq!(table.len(), 6);
}

#[test]
fn test_retain_with_no_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut _| false);
    assert_eq!(table.len(), 0);
}

