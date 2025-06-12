// Answer 0

#[test]
fn test_is_empty_on_empty_map() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.is_empty();
}

#[test]
fn test_is_empty_on_non_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 2);
    map.is_empty();
}

