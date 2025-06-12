// Answer 0

#[test]
fn test_iter_empty_table() {
    let table: HashTable<u32> = HashTable::new_in(Global);
    let iter = table.iter();
}

#[test]
fn test_iter_single_element() {
    let mut table: HashTable<&str> = HashTable::with_capacity_in(1, Global);
    let hasher = |val: &_| val.len() as u64;
    table.insert_unique(hasher("a"), "a", hasher);
    let iter = table.iter();
}

#[test]
fn test_iter_multiple_elements() {
    let mut table: HashTable<&str> = HashTable::with_capacity_in(5, Global);
    let hasher = |val: &_| val.len() as u64;
    table.insert_unique(hasher("a"), "a", hasher);
    table.insert_unique(hasher("b"), "b", hasher);
    table.insert_unique(hasher("c"), "c", hasher);
    let iter = table.iter();
}

#[test]
fn test_iter_after_clear() {
    let mut table: HashTable<&str> = HashTable::with_capacity_in(3, Global);
    let hasher = |val: &_| val.len() as u64;
    table.insert_unique(hasher("a"), "a", hasher);
    table.insert_unique(hasher("b"), "b", hasher);
    table.clear();
    let iter = table.iter();
}

#[test]
fn test_iter_edge_capacity() {
    let mut table: HashTable<u32> = HashTable::with_capacity_in(1_000_000, Global);
    let hasher = |val: &_| *val;
    for i in 0..1_000_000 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    let iter = table.iter();
}

