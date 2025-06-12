// Answer 0

#[test]
fn test_retain_even_numbers() {
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
fn test_retain_no_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 1..=5 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x > 10);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_retain_all_elements() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 1..=5 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x < 10);
    assert_eq!(table.len(), 5);
}

#[test]
fn test_retain_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<i32, i32, DefaultHashBuilder> = HashTable::new();
    table.retain(|&mut _x| true);
    assert_eq!(table.len(), 0);
}

