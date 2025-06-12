// Answer 0

#[test]
fn test_into_iter_new() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: i32,
    }
    
    let entries = vec![
        TestBucket { hash: 1, key: 1, value: 10 },
        TestBucket { hash: 2, key: 2, value: 20 },
    ];
  
    let iter: IntoIter<TestBucket> = IntoIter::new(entries);

    let mut collected: Vec<TestBucket> = iter.iter.collect();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].key, 1);
    assert_eq!(collected[1].key, 2);
}

#[test]
fn test_into_iter_new_empty() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: i32,
    }
    
    let entries: Vec<TestBucket> = Vec::new();
    let iter: IntoIter<TestBucket> = IntoIter::new(entries);
    
    let collected: Vec<TestBucket> = iter.iter.collect();
    assert_eq!(collected.len(), 0);
}

