// Answer 0

#[test]
fn test_truncate_valid_range() {
    let mut map_core = IndexMapCore::<usize, usize>::with_capacity(10);
    for i in 0..10 {
        map_core.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    map_core.truncate(5);
}

#[test]
fn test_truncate_edge_case() {
    let mut map_core = IndexMapCore::<usize, usize>::with_capacity(100);
    for i in 0..100 {
        map_core.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    map_core.truncate(50);
}

#[test]
fn test_truncate_small_size() {
    let mut map_core = IndexMapCore::<usize, usize>::new();
    for i in 0..3 {
        map_core.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    map_core.truncate(2);
}

#[test]
fn test_truncate_zero() {
    let mut map_core = IndexMapCore::<usize, usize>::new();
    for i in 0..5 {
        map_core.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    map_core.truncate(1);
}

#[test]
fn test_truncate_random_large_size() {
    let mut map_core = IndexMapCore::<usize, usize>::with_capacity(50);
    for i in 0..50 {
        map_core.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }
    map_core.truncate(30);
}

