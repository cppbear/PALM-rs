// Answer 0

#[test]
fn test_reserve_entries_increase_capacity() {
    struct TestBucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }

    let mut entries: Vec<TestBucket<usize, usize>> = Vec::new();
    entries.push(TestBucket { hash: HashValue::default(), key: 1, value: 10 });
    entries.push(TestBucket { hash: HashValue::default(), key: 2, value: 20 });

    let initial_capacity = entries.len();
    let additional = 2;
    let try_capacity = 6;

    reserve_entries(&mut entries, additional, try_capacity);

    assert!(entries.len() == initial_capacity + additional);
}

#[test]
fn test_reserve_entries_no_increase_capacity() {
    struct TestBucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }

    let mut entries: Vec<TestBucket<usize, usize>> = Vec::new();
    entries.push(TestBucket { hash: HashValue::default(), key: 1, value: 10 });

    let initial_capacity = entries.len();
    let additional = 1;
    let try_capacity = 1;

    reserve_entries(&mut entries, additional, try_capacity);

    assert!(entries.len() == initial_capacity);
}

#[test]
fn test_reserve_entries_exceeding_max_capacity() {
    struct TestBucket<K, V> {
        hash: HashValue,
        key: K,
        value: V,
    }

    let mut entries: Vec<TestBucket<usize, usize>> = Vec::new();
    let max_capacity = IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY;
    
    for i in 0..max_capacity {
        entries.push(TestBucket { hash: HashValue::default(), key: i, value: i as usize * 10 });
    }

    let initial_capacity = entries.len();
    let additional = 1;
    let try_capacity = max_capacity + 1;

    // This should panic since we're trying to exceed the defined maximum capacity.
    let result = std::panic::catch_unwind(|| {
        reserve_entries(&mut entries, additional, try_capacity);
    });

    assert!(result.is_err());
    assert!(entries.len() == initial_capacity);
}

