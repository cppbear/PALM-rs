// Answer 0

#[test]
fn test_into_keys_empty_map() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let keys = map.into_keys();
}

#[test]
fn test_into_keys_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.try_reserve(1).unwrap();
    map.insert(1, 2);
    let keys = map.into_keys();
}

#[test]
fn test_into_keys_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    for i in 0..5 {
        map.insert(i, i * 2);
    }
    let keys = map.into_keys();
}

#[test]
fn test_into_keys_large_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1000, RandomState::new());
    for i in 0..1000 {
        map.insert(i, i * 2);
    }
    let keys = map.into_keys();
}

#[test]
fn test_into_keys_panic_on_empty_split() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.try_reserve(5).unwrap();
    map.split_off(0);
}

#[test]
#[should_panic]
fn test_into_keys_panic_on_exceeding_split() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map.try_reserve(3).unwrap();
    let _items = map.split_off(4); // Panic expected
}

