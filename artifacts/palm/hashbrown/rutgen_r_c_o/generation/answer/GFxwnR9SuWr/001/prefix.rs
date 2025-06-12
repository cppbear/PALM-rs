// Answer 0

#[test]
fn test_insert_unique_basic() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(1, 1, hasher);
}

#[test]
fn test_insert_unique_multiple() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(2, 2, hasher);
    table.insert_unique(3, 3, hasher);
}

#[test]
fn test_insert_unique_large_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(u64::MAX, 1, hasher);
}

#[test]
fn test_insert_unique_edge_case_zero() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(1, 0, hasher);
}

#[test]
fn test_insert_unique_with_different_types() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &String| {
        let bytes = val.as_bytes();
        bytes.iter().map(|b| *b as u64).sum()
    };
    table.insert_unique(4, String::from("test"), hasher);
}

#[test]
fn test_insert_unique_with_negative_hash() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &i32| val.wrapping_neg() as u64;
    table.insert_unique(1, -1, hasher);
}

