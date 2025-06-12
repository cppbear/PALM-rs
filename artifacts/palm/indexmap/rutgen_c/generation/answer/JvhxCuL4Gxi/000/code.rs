// Answer 0

#[test]
fn test_reserve_exact_increases_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve_exact(5);
    assert_eq!(index_map.indices.capacity(), 5);
    assert_eq!(index_map.entries.capacity(), 5);
}

#[test]
fn test_reserve_exact_for_zero_additional() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve_exact(0);
    assert_eq!(index_map.indices.capacity(), 0);
    assert_eq!(index_map.entries.capacity(), 0);
}

#[test]
#[should_panic(expected = "capacity overflow")]
fn test_reserve_exact_panics_on_overflow() {
    let mut index_map: IndexMapCore<u32, u32> = IndexMapCore::new();
    index_map.reserve_exact(IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY + 1);
}

#[test]
fn test_reserve_exact_increases_capacity_with_existing_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    index_map.reserve_exact(3);
    assert_eq!(index_map.indices.capacity(), 3);
    assert_eq!(index_map.entries.capacity(), 3);
}

