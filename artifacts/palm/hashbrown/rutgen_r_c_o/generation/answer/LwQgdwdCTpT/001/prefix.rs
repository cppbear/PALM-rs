// Answer 0

#[test]
fn test_find_mut_with_existing_entry() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| (*val as u64);
    table.insert_unique(hasher(&1), 1, &hasher);
    let hash = hasher(&1);
    let mut result = table.find_mut(hash, |val| *val == 1);
    if let Some(value) = result {
        *value += 1;
    }
}

#[test]
fn test_find_mut_with_non_existing_entry() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| (*val as u64);
    table.insert_unique(hasher(&1), 1, &hasher);
    let hash = hasher(&2);
    let result = table.find_mut(hash, |val| *val == 2);
    assert!(result.is_none());
}

#[test]
fn test_find_mut_empty_table() {
    let mut table = HashTable::new_in(Global);
    let hash = 0u64;
    let result = table.find_mut(hash, |val| *val == 1);
    assert!(result.is_none());
}

#[test]
fn test_find_mut_updates_value() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| (*val as u64);
    table.insert_unique(hasher(&1), 1, &hasher);
    let hash = hasher(&1);
    let result = table.find_mut(hash, |val| *val == 1);
    if let Some(value) = result {
        *value = 2;
    }
    let new_result = table.find_mut(hash, |val| *val == 2);
    assert!(new_result.is_some());
}

#[test]
fn test_find_mut_edge_case_zero_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| (*val as u64);
    table.insert_unique(0, 1, &hasher);
    let result = table.find_mut(0, |val| *val == 1);
    if let Some(value) = result {
        *value += 1;
    }
}

#[test]
fn test_find_mut_edge_case_max_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| (*val as u64);
    table.insert_unique(u64::MAX, 1, &hasher);
    let result = table.find_mut(u64::MAX, |val| *val == 1);
    if let Some(value) = result {
        *value += 1;
    }
}

