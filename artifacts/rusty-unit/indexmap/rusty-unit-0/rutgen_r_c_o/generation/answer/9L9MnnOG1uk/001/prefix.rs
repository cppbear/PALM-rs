// Answer 0

#[test]
fn test_values_mut_empty() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _iterator = map.values_mut();
}

#[test]
fn test_values_mut_single_element() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    let _iterator = map.values_mut();
}

#[test]
fn test_values_mut_multiple_elements() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let _iterator = map.values_mut();
}

#[test]
fn test_values_mut_truncation() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.truncate(2);
    let _iterator = map.values_mut();
}

#[test]
fn test_values_mut_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(100, RandomState::new());
    for i in 0..50 {
        map.insert(i, i * 10);
    }
    let _iterator = map.values_mut();
}

#[test]
fn test_values_mut_reserve() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.reserve(10);
    let _iterator = map.values_mut();
}

#[test]
fn test_values_mut_shrink_to_fit() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.shrink_to_fit();
    let _iterator = map.values_mut();
}

