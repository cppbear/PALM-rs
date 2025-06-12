// Answer 0

#[test]
fn test_is_empty_on_new_table() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.is_empty();
}

#[test]
fn test_is_empty_after_insertion() {
    let hasher = |val: &_| val.hash();
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(1, 1, hasher);
    table.is_empty();
}

#[test]
fn test_is_empty_after_clear() {
    let hasher = |val: &_| val.hash();
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(2, 2, hasher);
    table.clear();
    table.is_empty();
}

#[test]
fn test_is_empty_after_multiple_insertions() {
    let hasher = |val: &_| val.hash();
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(3, 3, hasher);
    table.insert_unique(4, 4, hasher);
    table.is_empty();
}

#[test]
fn test_is_empty_on_large_capacity() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(1 << 30, Global);
    table.is_empty();
}

#[test]
fn test_is_empty_on_large_additional() {
    let hasher = |val: &_| val.hash();
    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    for i in 0..10 {
        table.insert_unique(i as u64, i, hasher);
    }
    table.is_empty();
}

