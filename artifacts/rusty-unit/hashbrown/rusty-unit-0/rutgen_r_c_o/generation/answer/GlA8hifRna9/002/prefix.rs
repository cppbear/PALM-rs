// Answer 0

#[test]
fn test_find_entry_not_found() {
    let mut table = HashTable::new_in(Global);
    let hash = 0;
    let eq = |val| val == &9999;

    let result = table.find_entry(hash, eq);
}

#[test]
fn test_find_entry_empty_table() {
    let mut table = HashTable::new_in(Global);
    let hash = 0;
    let eq = |val| val == &1;

    let result = table.find_entry(hash, eq);
}

#[test]
fn test_find_entry_non_matching_hash() {
    let mut table = HashTable::new_in(Global);
    let hash = 0;
    let eq = |val| val == &42;

    table.insert_unique(1, (1, "a"), |val| val.0);
    let result = table.find_entry(hash, eq);
}

#[test]
fn test_find_entry_non_matching_value() {
    let mut table = HashTable::new_in(Global);
    let hash = 1;
    let eq = |val| val == &42;

    table.insert_unique(hash, (1, "a"), |val| val.0);
    let result = table.find_entry(hash, eq);
}

