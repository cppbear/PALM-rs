// Answer 0

#[test]
fn test_retain_all_elements_kept() {
    let mut table = HashTable::new();
    let hasher = |val: &_| val;
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x > 0); // Retain all elements
}

#[test]
fn test_retain_no_elements_kept() {
    let mut table = HashTable::new();
    let hasher = |val: &_| val;
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x < 0); // Retain no elements
}

#[test]
fn test_retain_some_elements_kept() {
    let mut table = HashTable::new();
    let hasher = |val: &_| val;
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x % 2 == 0); // Retain even elements
}

#[test]
fn test_retain_empty_table() {
    let mut table = HashTable::new();
    table.retain(|&mut x| x % 2 == 0); // Should handle empty gracefully
}

#[test]
fn test_retain_with_non_removable_elements() {
    let mut table = HashTable::new();
    let hasher = |val: &_| val;
    for x in [1, 2, 3, 4, 5, 6] {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x != 3); // Retain all except 3
}

