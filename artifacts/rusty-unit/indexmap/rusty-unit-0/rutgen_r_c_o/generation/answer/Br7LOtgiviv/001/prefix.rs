// Answer 0

#[test]
fn test_new_empty_entries() {
    let entries: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = IntoIter::new(entries);
}

#[test]
fn test_new_single_entry() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 1, value: 100 }];
    let iter = IntoIter::new(entries);
}

#[test]
fn test_new_multiple_entries() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 100 },
        Bucket { hash: HashValue::default(), key: 2, value: 200 },
        Bucket { hash: HashValue::default(), key: 3, value: 300 },
    ];
    let iter = IntoIter::new(entries);
}

#[test]
fn test_new_large_entries() {
    let mut entries = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        entries.push(Bucket { hash: HashValue::default(), key: i, value: i * 10 });
    }
    let iter = IntoIter::new(entries);
}

#[test]
fn test_new_edge_case_one_element() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 42, value: 420 }];
    let iter = IntoIter::new(entries);
}

#[test]
#[should_panic]
fn test_new_exceeding_memory_limit() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 0, value: 0 }; 1_000_001];
    let iter = IntoIter::new(entries);
}

