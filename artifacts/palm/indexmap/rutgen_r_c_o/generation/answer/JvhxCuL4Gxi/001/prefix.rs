// Answer 0

#[test]
fn test_reserve_exact_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve_exact(0);
}

#[test]
fn test_reserve_exact_minimal() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve_exact(1);
}

#[test]
fn test_reserve_exact_mid_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.reserve_exact(100);
}

#[test]
fn test_reserve_exact_large() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve_exact(IndexMapCore::MAX_ENTRIES_CAPACITY);
}

#[should_panic(expected = "exceeds the capacity limit")]
#[test]
fn test_reserve_exact_exceed_capacity() {
    let capacity = (isize::MAX as usize) / mem::size_of::<Bucket<usize, usize>>() + 1;
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve_exact(capacity);
}

