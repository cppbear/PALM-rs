// Answer 0

#[test]
fn test_drain_empty() {
    let mut table = HashTable::new_in(Global);
    let drain = table.drain();
}

#[test]
fn test_drain_single_element() {
    let mut table = HashTable::with_capacity_in(1, Global);
    let hasher = |val: &i32| *val as u64;    
    table.insert_unique(hasher(&1), 1, hasher);
    let drain = table.drain();
}

#[test]
fn test_drain_multiple_elements() {
    let mut table = HashTable::with_capacity_in(3, Global);
    let hasher = |val: &i32| *val as u64;    
    for x in 1..=3 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let drain = table.drain();
}

#[test]
fn test_drain_after_inserting_capacity_exceeded() {
    let mut table = HashTable::with_capacity_in(2, Global);
    let hasher = |val: &i32| *val as u64;    
    for x in 1..=3 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let drain = table.drain();
}

#[test]
fn test_drain_repeated_elements() {
    let mut table = HashTable::with_capacity_in(3, Global);
    let hasher = |val: &i32| *val as u64;    
    for _ in 0..3 {
        table.insert_unique(hasher(&1), 1, hasher);
    }
    let drain = table.drain();
}

