// Answer 0

#[test]
fn test_len_empty() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.len();
}

#[test]
fn test_len_one() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    map.len();
}

#[test]
fn test_len_two() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.len();
}

#[test]
fn test_len_three() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.len();
}

#[test]
fn test_len_ten() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    map.len();
}

#[test]
fn test_len_hundred() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(100, RandomState::new());
    for i in 0..100 {
        map.insert(i, i * 10);
    }
    map.len();
}

#[test]
fn test_len_thousand() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1000, RandomState::new());
    for i in 0..1000 {
        map.insert(i, i * 10);
    }
    map.len();
}

#[test]
fn test_len_ten_thousand() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10_000, RandomState::new());
    for i in 0..10_000 {
        map.insert(i, i * 10);
    }
    map.len();
}

#[test]
fn test_len_hundred_thousand() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(100_000, RandomState::new());
    for i in 0..100_000 {
        map.insert(i, i * 10);
    }
    map.len();
}

#[test]
fn test_len_million() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1_000_000, RandomState::new());
    for i in 0..1_000_000 {
        map.insert(i, i * 10);
    }
    map.len();
}

#[test]
fn test_len_large() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1_000_000_000, RandomState::new());
    for i in 0..1_000_000 {
        map.insert(i, i * 10);
    }
    map.len();
}

