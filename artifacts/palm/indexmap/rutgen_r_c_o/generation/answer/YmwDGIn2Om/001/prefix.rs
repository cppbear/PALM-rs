// Answer 0

#[test]
fn test_with_capacity_zero() {
    let result = IndexMapCore::<usize, usize>::with_capacity(0);
}

#[test]
fn test_with_capacity_small() {
    let result = IndexMapCore::<usize, usize>::with_capacity(1);
}

#[test]
fn test_with_capacity_mid_range() {
    let result = IndexMapCore::<usize, usize>::with_capacity(100);
}

#[test]
fn test_with_capacity_large() {
    let result = IndexMapCore::<usize, usize>::with_capacity(1_000_000);
}

#[test]
#[should_panic]
fn test_with_capacity_exceeds_max_entries_capacity() {
    let result = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY + 1);
}

