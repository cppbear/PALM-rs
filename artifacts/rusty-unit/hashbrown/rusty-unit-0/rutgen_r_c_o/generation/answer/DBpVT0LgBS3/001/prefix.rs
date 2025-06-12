// Answer 0

#[test]
fn test_extract_if_with_empty_table() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let _drain_iterator = table.extract_if(|_v| true);
}

#[test]
fn test_extract_if_with_even_numbers() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(8, Global);
    let hasher = |val: &i32| *val as u64;
    for x in 0..8 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let _drain_iterator = table.extract_if(|v| *v % 2 == 0);
}

#[test]
fn test_extract_if_with_odd_numbers() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(8, Global);
    let hasher = |val: &i32| *val as u64;
    for x in 1..9 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let _drain_iterator = table.extract_if(|v| *v % 2 != 0);
}

#[test]
fn test_extract_if_with_all_elements() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(16, Global);
    let hasher = |val: &i32| *val as u64;
    for x in 0..16 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let _drain_iterator = table.extract_if(|_v| true);
}

#[test]
fn test_extract_if_with_no_elements() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(1, Global);
    let _drain_iterator = table.extract_if(|_v| true);
}

#[test]
fn test_extract_if_with_full_table() {
    let mut table: HashTable<i32> = HashTable::with_capacity_in(16, Global);
    let hasher = |val: &i32| *val as u64;
    for x in 1..=16 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let _drain_iterator = table.extract_if(|v| *v < 10);
}

#[test]
fn test_extract_if_with_edge_case() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(hasher(&0), 0, hasher);
    table.insert_unique(hasher(&1), 1, hasher);
    let _drain_iterator = table.extract_if(|v| *v == 1);
}

