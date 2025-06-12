// Answer 0

#[test]
fn test_as_mut_slice_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let _slice = map.as_mut_slice();
}

#[test]
fn test_as_mut_slice_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    let _slice = map.as_mut_slice();
}

#[test]
fn test_as_mut_slice_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let _slice = map.as_mut_slice();
}

#[test]
fn test_as_mut_slice_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity(1024);
    for i in 0..1024 {
        map.insert(i, i * 10);
    }
    let _slice = map.as_mut_slice();
}

#[test]
fn test_as_mut_slice_large_entry() {
    let mut map: IndexMap<i32, Vec<i32>, RandomState> = IndexMap::new();
    for i in 0..1_000_000 {
        map.insert(i, vec![i; 10]);
    }
    let _slice = map.as_mut_slice();
}

