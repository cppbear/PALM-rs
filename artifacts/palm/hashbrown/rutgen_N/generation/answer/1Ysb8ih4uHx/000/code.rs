// Answer 0

#[test]
fn test_drain_on_empty_table() {
    use hashbrown::HashMap;

    let mut table: HashMap<i32, i32> = HashMap::new();
    let drain_iter = table.drain();
    let drained_elements: Vec<(i32, i32)> = drain_iter.collect();
    assert!(drained_elements.is_empty());
}

#[test]
fn test_drain_on_filled_table() {
    use hashbrown::HashMap;

    let mut table: HashMap<i32, i32> = HashMap::new();
    table.insert(1, 10);
    table.insert(2, 20);
    table.insert(3, 30);
    
    let drain_iter = table.drain();
    let drained_elements: Vec<(i32, i32)> = drain_iter.collect();
    
    assert_eq!(drained_elements.len(), 3);
    assert!(table.is_empty());
}

#[test]
fn test_drain_remains_ordered() {
    use hashbrown::HashMap;

    let mut table: HashMap<i32, i32> = HashMap::new();
    table.insert(1, 10);
    table.insert(2, 20);
    table.insert(3, 30);
    
    let drain_iter = table.drain();
    let drained_elements: Vec<(i32, i32)> = drain_iter.collect();
    
    assert_eq!(drained_elements, vec![(1, 10), (2, 20), (3, 30)]);
}

#[test]
#[should_panic]
fn test_drain_after_empty() {
    use hashbrown::HashMap;

    let mut table: HashMap<i32, i32> = HashMap::new();
    let drain_iter = table.drain();
    drain_iter.collect();
    
    // This should panic since the table is now empty
    let _ = table.drain();
}

