// Answer 0

#[test]
fn test_split_splice_empty() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let range = 0..0;
    map.split_splice(range);
}

#[test]
fn test_split_splice_full_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        map.push_entry(HashValue::from(i as u64), i, i * 10);
    }
    let range = 0..5;
    map.split_splice(range);
}

#[test]
fn test_split_splice_partial_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        map.push_entry(HashValue::from(i as u64), i, i * 10);
    }
    let range = 2..4;
    map.split_splice(range);
}

#[test]
#[should_panic]
fn test_split_splice_out_of_bounds_start() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        map.push_entry(HashValue::from(i as u64), i, i * 10);
    }
    let range = 6..8; // Out of bounds
    map.split_splice(range);
}

#[test]
#[should_panic]
fn test_split_splice_out_of_bounds_end() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        map.push_entry(HashValue::from(i as u64), i, i * 10);
    }
    let range = 0..6; // Out of bounds
    map.split_splice(range);
}

#[test]
fn test_split_splice_edge_case() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..3 {
        map.push_entry(HashValue::from(i as u64), i, i * 10);
    }
    let range = 1..2;
    map.split_splice(range);
}

#[test]
fn test_split_splice_single_element() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.push_entry(HashValue::from(0), 0, 0);
    let range = 0..1;
    map.split_splice(range);
}

