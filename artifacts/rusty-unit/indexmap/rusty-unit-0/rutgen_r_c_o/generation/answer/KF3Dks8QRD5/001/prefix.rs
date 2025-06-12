// Answer 0

#[test]
fn test_drain_empty_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.drain(0..0);
}

#[test]
fn test_drain_single_element() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    map.push_entry(1.into(), 0, 10);
    let _ = map.drain(0..1);
}

#[test]
fn test_drain_same_start_end() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    map.push_entry(1.into(), 0, 10);
    let _ = map.drain(1..1);
}

#[test]
fn test_drain_out_of_bounds_end() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    map.push_entry(1.into(), 0, 10);
    let _ = map.drain(0..2);
}

#[test]
fn test_drain_within_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    map.push_entry(1.into(), 0, 10);
    map.push_entry(2.into(), 1, 20);
    let _ = map.drain(0..2);
}

#[test]
fn test_drain_full_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY {
        map.push_entry((i + 1).into(), i, i as usize);
    }
    let _ = map.drain(0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

#[test]
#[should_panic(expected = "range start index 0 out of range for slice of length 0")]
fn test_drain_panic_start_out_of_bounds() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.drain(1..2);
}

#[test]
#[should_panic(expected = "range end index 0 out of range for slice of length 0")]
fn test_drain_panic_end_out_of_bounds() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.drain(0..1);
}

#[test]
#[should_panic(expected = "range start index 10 should be <= range end index 5")]
fn test_drain_panic_invalid_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.drain(10..5);
}

