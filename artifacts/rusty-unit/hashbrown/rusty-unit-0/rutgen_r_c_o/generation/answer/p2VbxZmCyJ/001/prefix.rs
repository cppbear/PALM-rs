// Answer 0

#[test]
fn test_clear_non_empty_hash_table() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val.hash();
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.clear();
}

#[test]
fn test_clear_hash_table_with_capacity() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.hash();
    for i in 0..50 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.clear();
}

#[test]
fn test_clear_large_hash_table() {
    let mut table = HashTable::with_capacity_in(1000000, Global);
    let hasher = |val: &_| val.hash();
    for i in 0..1000000 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.clear();
}

#[test]
fn test_clear_empty_hash_table() {
    let mut table = HashTable::new_in(Global);
    table.clear();
}

#[test]
fn test_clear_hash_table_with_single_entry() {
    let mut table = HashTable::with_capacity_in(1, Global);
    let hasher = |val: &_| val.hash();
    table.insert_unique(hasher(&1), 1, hasher);
    table.clear();
}

