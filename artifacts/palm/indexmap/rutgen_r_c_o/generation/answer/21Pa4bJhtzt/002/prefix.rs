// Answer 0

#[test]
fn test_reserve_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve(0);
}

#[test]
fn test_reserve_exact_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.reserve(10);
}

#[test]
fn test_reserve_below_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.reserve(5);
}

#[test]
fn test_reserve_edge_case() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    map.reserve(0);
}

#[test]
fn test_reserve_no_growth_needed() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    map.reserve(0);
    map.reserve(5);
    map.reserve(5);
}

