// Answer 0

#[test]
fn test_try_reserve_exact_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.try_reserve_exact(0);
}

#[test]
fn test_try_reserve_exact_small_value() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.try_reserve_exact(1);
}

#[test]
fn test_try_reserve_exact_large_value() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.try_reserve_exact(10);
}

#[test]
#[should_panic]
fn test_try_reserve_exact_exceed_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.try_reserve_exact(IndexMapCore::MAX_ENTRIES_CAPACITY);
} 

#[test]
fn test_try_reserve_exact_edge_case() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.try_reserve_exact(4294967295);
}

