// Answer 0

#[test]
fn test_first_empty_map() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.first();
}

#[test]
fn test_first_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.first();
}

#[test]
fn test_first_multiple_entries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.first();
}

#[test]
fn test_first_at_max_size() {
    let max_size = 100;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    for i in 0..max_size {
        map.insert(i, i * 10);
    }
    map.first();
}

#[test]
fn test_first_with_one_exceeding_max_size() {
    let max_size = 100;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    for i in 0..max_size {
        map.insert(i, i * 10);
    }
    map.insert(max_size, max_size * 10);
    map.first();
}

