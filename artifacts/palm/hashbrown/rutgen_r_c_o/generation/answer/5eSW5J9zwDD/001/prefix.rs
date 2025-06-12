// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let mut iter = table.iter_mut();
}

#[test]
fn test_iter_mut_single_element() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &_| val.clone() as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    let mut iter = table.iter_mut();
}

#[test]
fn test_iter_mut_multiple_elements() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &_| val.clone() as u64;
    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    let mut iter = table.iter_mut();
}

#[test]
fn test_iter_mut_large_table() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &_| val.clone() as u64;
    for i in 1..=10000 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    let mut iter = table.iter_mut();
}

#[test]
fn test_iter_mut_update_values() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &_| val.clone() as u64;
    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    
    for val in table.iter_mut() {
        *val += 1;
    }
    let mut iter = table.iter_mut();
}

