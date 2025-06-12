// Answer 0

#[test]
fn test_shrink_to_fit_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_small() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_medium() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(50, RandomState::new());
    for i in 10..60 {
        map.insert(i, i * 3);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_large() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(500, RandomState::new());
    for i in 1000..1500 {
        map.insert(i, i * 4);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_xlarge() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10000, RandomState::new());
    for i in 10000..20000 {
        map.insert(i, i * 5);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_max_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(100000, RandomState::new());
    for i in 50000..60000 {
        map.insert(i, i * 6);
    }
    map.shrink_to_fit();
}

