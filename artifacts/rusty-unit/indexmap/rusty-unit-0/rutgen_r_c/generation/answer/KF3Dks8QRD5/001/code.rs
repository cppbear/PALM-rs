// Answer 0

#[test]
fn test_drain_valid_range() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    // Manually populating the entries to ensure we have a scenario to drain
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 0 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 2 });

    let drained: Vec<Bucket<usize, usize>> = index_map.drain(1..3).collect();

    assert_eq!(drained.len(), 2);
    assert_eq!(drained[0].key, 1);
    assert_eq!(drained[1].key, 2);
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, 0);
}

#[test]
#[should_panic(expected = "range start index 3 out of range for slice of length 3")]
fn test_drain_out_of_bounds_start() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 0 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
    
    // This should panic as we're trying to access an index out of bounds
    index_map.drain(3..4);
}

#[test]
#[should_panic(expected = "range end index 0 out of range for slice of length 3")]
fn test_drain_out_of_bounds_end() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 0 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 1 });
    
    // This should panic as we're trying to access an end index out of bounds
    index_map.drain(0..0);
}

#[test]
#[should_panic(expected = "range start index 1 should be <= range end index 0")]
fn test_drain_invalid_range() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 0 });

    // This should panic as the start is greater than the end
    index_map.drain(1..0);
}

