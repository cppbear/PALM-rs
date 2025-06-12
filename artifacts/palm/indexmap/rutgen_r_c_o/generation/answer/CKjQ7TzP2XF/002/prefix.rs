// Answer 0

#[test]
fn test_try_reserve_exact_zero_additional() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.try_reserve_exact(0);
}

#[test]
fn test_try_reserve_exact_min_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let _ = map.try_reserve_exact(1);
}

#[test]
fn test_try_reserve_exact_mid_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    let _ = map.try_reserve_exact(3);
}

#[test]
fn test_try_reserve_exact_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY - 1);
    let _ = map.try_reserve_exact(1);
}

#[test]
#[should_panic]
fn test_try_reserve_exact_exceed_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.try_reserve_exact(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

