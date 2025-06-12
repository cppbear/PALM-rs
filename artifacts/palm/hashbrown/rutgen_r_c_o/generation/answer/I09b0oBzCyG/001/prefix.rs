// Answer 0

#[test]
fn test_len_empty() {
    let mut table = HashTable::new_in(Global);
    let length = table.len();
}

#[test]
fn test_len_single_element() {
    let hasher = |val: &_| val.hash();
    let mut table = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    let length = table.len();
}

#[test]
fn test_len_multiple_elements() {
    let hasher = |val: &_| val.hash();
    let mut table = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);
    let length = table.len();
}

#[test]
fn test_len_after_clear() {
    let hasher = |val: &_| val.hash();
    let mut table = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.clear();
    let length = table.len();
}

#[test]
fn test_len_large_number_of_elements() {
    let hasher = |val: &_| val.hash();
    let mut table = HashTable::with_capacity_in(1000000, Global);
    for i in 0..1000000 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    let length = table.len();
}

#[test]
fn test_len_after_removal() {
    let hasher = |val: &_| val.hash();
    let mut table = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.remove_entry(hasher(&1), |&val| val == 1);
    let length = table.len();
}

#[test]
fn test_len_empty_after_removal() {
    let hasher = |val: &_| val.hash();
    let mut table = HashTable::new_in(Global);
    table.insert_unique(hasher(&1), 1, hasher);
    table.remove_entry(hasher(&1), |&val| val == 1);
    let length = table.len();
}

