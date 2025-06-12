// Answer 0

#[test]
fn test_retain_with_all_even_elements() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val;

    for x in 0..=10 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x % 2 == 0);
}

#[test]
fn test_retain_with_all_odd_elements() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val;

    for x in 1..=11 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x % 2 == 0);
}

#[test]
fn test_retain_with_mixed_elements() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val;

    for x in 0..=10 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x % 3 == 0);
}

#[test]
fn test_retain_with_no_elements() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val;

    table.retain(|&mut _| false);
}

#[test]
fn test_retain_with_all_elements_retained() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val;

    for x in 0..=10 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    
    table.retain(|&mut _| true);
}

#[test]
fn test_retain_with_capacity_zero() {
    let mut table: HashTable<u32, Global> = HashTable::with_capacity_in(0, Global);
    
    table.retain(|&mut _| false);
}

#[test]
fn test_retain_edge_case_large_capacity() {
    let mut table = HashTable::with_capacity_in(1000, Global);
    let hasher = |val: &_| val;

    for x in 0..=100 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x <= 50);
}

