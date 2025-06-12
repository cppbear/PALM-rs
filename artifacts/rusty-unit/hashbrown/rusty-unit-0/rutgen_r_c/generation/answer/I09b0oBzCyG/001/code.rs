// Answer 0

#[test]
fn test_len_empty() {
    let hasher = |val: &_| val.clone() as u64; // Simple identity hash
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    assert_eq!(table.len(), 0);
}

#[test]
fn test_len_after_insert() {
    let hasher = |val: &_| val.clone() as u64; // Simple identity hash
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    assert_eq!(table.len(), 1);
}

#[test]
fn test_len_multiple_inserts() {
    let hasher = |val: &_| val.clone() as u64; // Simple identity hash
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);
    assert_eq!(table.len(), 3);
}

#[test]
fn test_len_after_clear() {
    let hasher = |val: &_| val.clone() as u64; // Simple identity hash
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    assert_eq!(table.len(), 1);
    table.clear();
    assert_eq!(table.len(), 0);
}

#[test]
fn test_len_with_duplicates() {
    let hasher = |val: &_| val.clone() as u64; // Simple identity hash
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&1), 1, hasher); // Duplicate insert
    assert_eq!(table.len(), 1);
}

