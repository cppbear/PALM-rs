// Answer 0

#[test]
fn test_len_empty() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.len();
}

#[test]
fn test_len_small() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices.push(0);
    map.len();
}

#[test]
fn test_len_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        map.indices.push(i);
    }
    map.len();
}

#[test]
fn test_len_after_clear() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices.push(1);
    map.clear();
    map.len();
}

#[test]
fn test_len_after_truncate() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices.push(1);
    map.truncate(0);
    map.len();
}

