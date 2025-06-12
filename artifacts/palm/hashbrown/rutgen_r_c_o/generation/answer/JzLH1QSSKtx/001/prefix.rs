// Answer 0

#[test]
fn test_new_hash_table_zero_capacity() {
    let table: HashTable<&str> = HashTable::new();
}

#[test]
fn test_new_hash_table_with_capacity_zero() {
    let table: HashTable<&str> = HashTable::with_capacity(0);
}

#[test]
fn test_new_hash_table_with_small_capacity() {
    let table: HashTable<&str> = HashTable::with_capacity(1);
}

#[test]
fn test_new_hash_table_with_large_capacity() {
    let table: HashTable<&str> = HashTable::with_capacity(1_000_000);
}

#[test]
fn test_new_hash_table_with_maximum_capacity() {
    let table: HashTable<&str> = HashTable::with_capacity(u32::MAX as usize);
}

#[test]
#[should_panic]
fn test_new_hash_table_with_negative_capacity() {
    // This test is theoretical since usize can't be negative; no runtime check exists.
    let _table: HashTable<&str> = HashTable::with_capacity(!0);
}

