// Answer 0

#[test]
fn test_index_map_new_empty() {
    let map: IndexMap<i32, i32> = IndexMap::new();
}

#[test]
fn test_index_map_with_capacity_zero() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(0);
}

#[test]
fn test_index_map_with_small_capacity() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(1);
}

#[test]
fn test_index_map_with_large_capacity() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(1000);
}

#[test]
fn test_index_map_with_maximum_capacity() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(1_000_000);
}

#[test]
#[should_panic]
fn test_index_map_with_negative_capacity() {
    let map: IndexMap<i32, i32> = IndexMap::with_capacity(-1);
}

