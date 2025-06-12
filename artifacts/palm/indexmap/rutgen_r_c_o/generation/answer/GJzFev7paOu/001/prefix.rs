// Answer 0

#[test]
fn test_drain_full_range() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    let _drain = map.drain(..);
}

#[test]
fn test_drain_start_to_end() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    let _drain = map.drain(0..3);
}

#[test]
fn test_drain_partially() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    let _drain = map.drain(1..3);
}

#[test]
#[should_panic]
fn test_drain_invalid_range_start_greater_than_end() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "a");
    map.insert(2, "b");
    let _drain = map.drain(2..1);
}

#[test]
#[should_panic]
fn test_drain_end_out_of_bounds() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "a");
    map.insert(2, "b");
    let _drain = map.drain(0..3);
}

#[test]
#[should_panic]
fn test_drain_n_minus_one_to_n_plus_one() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "a");
    map.insert(2, "b");
    let _drain = map.drain(1..4);
}

