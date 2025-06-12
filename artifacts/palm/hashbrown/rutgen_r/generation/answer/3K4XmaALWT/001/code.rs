// Answer 0

#[test]
fn test_allocation_size_empty_table() {
    let table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    assert_eq!(table.allocation_size(), 0);
}

#[test]
fn test_allocation_size_single_insertion() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    table.insert(1, 10);
    assert!(table.allocation_size() > 0);
}

#[test]
fn test_allocation_size_multiple_insertions() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    for i in 0..100 {
        table.insert(i, i * 10);
    }
    assert!(table.allocation_size() > 0);
}

#[test]
fn test_allocation_size_large_scale() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    for i in 0..1000 {
        table.insert(i, i * 10);
    }
    assert!(table.allocation_size() > 0);
}

#[test]
fn test_allocation_size_after_removal() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    for i in 0..100 {
        table.insert(i, i * 10);
    }
    for i in 0..50 {
        table.remove(&i);
    }
    assert!(table.allocation_size() > 0);
}

