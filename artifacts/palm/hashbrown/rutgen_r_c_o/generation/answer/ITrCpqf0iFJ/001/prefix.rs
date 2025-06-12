// Answer 0

#[test]
fn test_iter_hash_mut_with_empty_table() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hash = 1;
    let _iter = table.iter_hash_mut(hash);
}

#[test]
fn test_iter_hash_mut_with_non_existing_hash() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&2), 3, hasher);
    let hash = 9999;
    let _iter = table.iter_hash_mut(hash);
}

#[test]
fn test_iter_hash_mut_with_single_entry() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(hasher(&1), 2, hasher);
    let hash = hasher(&1);
    let _iter = table.iter_hash_mut(hash);
}

#[test]
fn test_iter_hash_mut_with_multiple_entries_same_hash() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&1), 3, hasher);
    let hash = hasher(&1);
    let mut iter = table.iter_hash_mut(hash);
    let _first_value = iter.next();
    let _second_value = iter.next();
}

#[test]
fn test_iter_hash_mut_with_different_hashes() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(hasher(&1), 2, hasher);
    table.insert_unique(hasher(&2), 3, hasher);
    table.insert_unique(hasher(&3), 5, hasher);
    let hash = hasher(&2);
    let mut iter = table.iter_hash_mut(hash);
    let _first_value = iter.next();
}

#[test]
fn test_iter_hash_mut_with_full_capacity() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &i32| *val as u64;
    for i in 0..100 {
        table.insert_unique(hasher(&(i as i32)), i as i32 * 2, hasher);
    }
    let hash = hasher(&(50 as i32));
    let _iter = table.iter_hash_mut(hash);
}

