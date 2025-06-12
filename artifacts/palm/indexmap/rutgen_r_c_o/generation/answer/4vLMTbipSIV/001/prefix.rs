// Answer 0

#[test]
fn test_iter_empty() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _iter = map.iter();
}

#[test]
fn test_iter_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 100);
    let _iter = map.iter();
}

#[test]
fn test_iter_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    let _iter = map.iter();
}

#[test]
fn test_iter_large_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1_000_000, RandomState::new());
    for i in 0..1_000_000 {
        map.insert(i, i * 10);
    }
    let _iter = map.iter();
}

#[test]
fn test_iter_after_clear() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 10);
    }
    map.clear();
    let _iter = map.iter();
}

#[test]
fn test_iter_truncate() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    map.truncate(5);
    let _iter = map.iter();
}

#[test]
fn test_iter_reserve() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.reserve(5);
    for i in 0..5 {
        map.insert(i, i * 10);
    }
    let _iter = map.iter();
}

#[test]
fn test_iter_with_edge_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1_000_000, RandomState::new());
    map.reserve_exact(10_000);
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    let _iter = map.iter();
}

