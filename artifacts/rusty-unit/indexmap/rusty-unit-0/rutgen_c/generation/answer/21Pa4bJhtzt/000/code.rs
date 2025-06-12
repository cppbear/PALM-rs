// Answer 0

#[test]
fn test_reserve_empty() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve(10);
    assert_eq!(map.indices.capacity(), 10);
    assert_eq!(map.entries.len(), 0);
}

#[test]
fn test_reserve_with_existing_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    map.reserve(10);
    assert_eq!(map.indices.capacity(), 10);
    assert_eq!(map.entries.len(), 2);
}

#[test]
fn test_reserve_exact_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve_exact(5);
    assert_eq!(map.indices.capacity(), 5);
    assert_eq!(map.entries.len(), 0);
}

#[test]
fn test_reserve_more_than_current_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.reserve(10); // Reserve more than current capacity
    assert_eq!(map.indices.capacity(), 10);
}

#[test]
fn test_reserve_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve(0); // Reserving zero should not change the capacity
    assert_eq!(map.indices.capacity(), 0);
}

