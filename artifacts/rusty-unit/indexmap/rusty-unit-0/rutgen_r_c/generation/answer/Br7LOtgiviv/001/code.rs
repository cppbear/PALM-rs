// Answer 0

#[test]
fn test_into_iter_new_with_empty_vector() {
    let entries: Vec<Bucket<i32>> = Vec::new();
    let iterator = IntoIter::new(entries);
    assert!(iterator.iter.as_slice().len() == 0);
}

#[test]
fn test_into_iter_new_with_single_entry() {
    let bucket = Bucket {
        hash: HashValue::default(),
        key: 42,
        value: "value",
    };
    let entries = vec![bucket];
    let iterator = IntoIter::new(entries);
    assert_eq!(iterator.iter.as_slice().len(), 1);
    assert_eq!(iterator.iter.as_slice()[0].key, 42);
}

#[test]
fn test_into_iter_new_with_multiple_entries() {
    let bucket1 = Bucket {
        hash: HashValue::default(),
        key: 1,
        value: "first",
    };
    let bucket2 = Bucket {
        hash: HashValue::default(),
        key: 2,
        value: "second",
    };
    let entries = vec![bucket1, bucket2];
    let iterator = IntoIter::new(entries);
    assert_eq!(iterator.iter.as_slice().len(), 2);
    assert_eq!(iterator.iter.as_slice()[0].key, 1);
    assert_eq!(iterator.iter.as_slice()[1].key, 2);
}

