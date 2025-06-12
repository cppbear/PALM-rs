// Answer 0

#[test]
fn test_split_off_valid_index() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut index_map = IndexMapCore::<usize, usize>::new();
    for i in 0..5 {
        index_map.push_entry(i as HashValue, i, i * 10);
    }
    
    let split_map = index_map.split_off(2);
    assert_eq!(split_map.entries.len(), 3);
    assert_eq!(index_map.entries.len(), 2);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 2 but the index is 3. Expected index <= len")]
fn test_split_off_out_of_bounds_index() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    for i in 0..2 {
        index_map.push_entry(i as HashValue, i, i * 10);
    }

    let _ = index_map.split_off(3);
}

#[test]
fn test_split_off_zero_index() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    for i in 0..3 {
        index_map.push_entry(i as HashValue, i, i * 10);
    }

    let split_map = index_map.split_off(0);
    assert_eq!(split_map.entries.len(), 3);
    assert_eq!(index_map.entries.len(), 0);
}

