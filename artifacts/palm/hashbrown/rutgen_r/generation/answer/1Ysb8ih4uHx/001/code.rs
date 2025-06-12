// Answer 0

#[test]
fn test_drain_empty_table() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    let drain_iter = table.drain();
    let collected: Vec<_> = drain_iter.collect();
    assert!(collected.is_empty());
}

#[test]
fn test_drain_single_element() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    table.insert(1, 100);
    let drain_iter = table.drain();
    let collected: Vec<_> = drain_iter.collect();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0], (1, 100));
}

#[test]
fn test_drain_multiple_elements() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    table.insert(1, 100);
    table.insert(2, 200);
    table.insert(3, 300);
    let drain_iter = table.drain();
    let collected: Vec<_> = drain_iter.collect();
    assert_eq!(collected.len(), 3);
    assert!(collected.contains(&(1, 100)));
    assert!(collected.contains(&(2, 200)));
    assert!(collected.contains(&(3, 300)));
}

#[test]
fn test_drain_after_drain() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    table.insert(1, 100);
    table.insert(2, 200);
    let _drain_iter = table.drain(); // First drain should remove elements
    let drain_iter_second = table.drain(); // Second drain should be empty
    let collected: Vec<_> = drain_iter_second.collect();
    assert!(collected.is_empty());
}

#[should_panic]
fn test_drain_from_drain_iterator() {
    let mut table: hashbrown::HashMap<i32, i32> = hashbrown::HashMap::new();
    table.insert(1, 100);
    let drain_iter = table.drain();
    let _drain_second = drain_iter.collect::<Vec<_>>(); // Collecting should not panic
    let _panicking_drain = drain_iter.collect::<Vec<_>>(); // This should panic since iter is consumed
}

