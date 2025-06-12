// Answer 0

#[test]
fn test_retain_all_elements_removed() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &_| val; // Identity function for simplicity

    for x in (0..=6).step_by(2) {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x % 2 == 1);  // Retain only odd numbers
}

#[test]
fn test_retain_some_elements_removed() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &_| val;

    for x in 0..=4 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x != 2);  // Retain all except 2
}

#[test]
fn test_retain_none_elements_removed() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &_| val;

    for x in 0..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x > 6);  // No elements satisfy the predicate
}

#[test]
fn test_retain_update_elements() {
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &_| val;

    for x in 1..=4 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| { x *= 2; x <= 6 }); // Change elements while checking condition
}

#[test]
fn test_retain_empty_table() {
    let mut table = HashTable::new_in(Global);
    table.retain(|&mut _| false);  // Retain on an empty table
}

#[test]
fn test_retain_capacity_limit() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val;

    for x in 0..100 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x % 3 == 0);  // Retain only multiples of 3
}

