// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _iter = map.iter_mut();
}

#[test]
fn test_iter_mut_single_element() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    let _iter = map.iter_mut();
}

#[test]
fn test_iter_mut_multiple_elements() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let _iter = map.iter_mut();
}

#[test]
fn test_iter_mut_large_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1_000_000, RandomState::new());
    for i in 0..1_000_000 {
        map.insert(i, i * 10);
    }
    let _iter = map.iter_mut();
}

#[test]
fn test_iter_mut_after_clear() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.clear();
    let _iter = map.iter_mut();
}

#[test]
fn test_iter_mut_truncate() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.truncate(1);
    let _iter = map.iter_mut();
}

