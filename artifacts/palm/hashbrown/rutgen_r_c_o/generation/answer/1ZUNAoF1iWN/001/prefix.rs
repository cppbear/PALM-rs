// Answer 0

#[test]
fn test_shrink_to_with_min_capacity_below_current_capacity() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.clone() as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.shrink_to(50, hasher);
}

#[test]
fn test_shrink_to_with_min_capacity_equal_to_current_capacity() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.clone() as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.shrink_to(100, hasher);
}

#[test]
fn test_shrink_to_with_min_capacity_zero() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.clone() as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    table.shrink_to(0, hasher);
}

#[should_panic]
fn test_shrink_to_with_min_capacity_greater_than_current_capacity() {
    let mut table = HashTable::with_capacity_in(50, Global);
    let hasher = |val: &_| val.clone() as u64;
    table.shrink_to(100, hasher);
}

#[test]
fn test_shrink_to_with_min_capacity_equal_to_one() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.clone() as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    table.shrink_to(1, hasher);
}

#[test]
fn test_shrink_to_with_multiple_entries() {
    let mut table = HashTable::with_capacity_in(200, Global);
    let hasher = |val: &_| val.clone() as u64;
    for i in 1..=50 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.shrink_to(20, hasher);
}

#[test]
fn test_shrink_to_with_large_min_capacity() {
    let mut table = HashTable::with_capacity_in(1000, Global);
    let hasher = |val: &_| val.clone() as u64;
    table.shrink_to(500, hasher);
}

