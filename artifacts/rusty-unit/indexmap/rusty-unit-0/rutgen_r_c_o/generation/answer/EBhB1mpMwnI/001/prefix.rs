// Answer 0

#[test]
fn test_reserve_exact_zero() {
    let mut map = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.reserve_exact(0);
}

#[test]
fn test_reserve_exact_small() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.reserve_exact(1);
}

#[test]
fn test_reserve_exact_medium() {
    let mut map = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.reserve_exact(10);
}

#[test]
fn test_reserve_exact_large() {
    let mut map = IndexMap::with_capacity_and_hasher(1000, RandomState::new());
    map.reserve_exact(500);
}

#[test]
#[should_panic]
fn test_reserve_exact_overflow() {
    let mut map = IndexMap::with_capacity_and_hasher(usize::MAX, RandomState::new());
    map.reserve_exact(1);
}

