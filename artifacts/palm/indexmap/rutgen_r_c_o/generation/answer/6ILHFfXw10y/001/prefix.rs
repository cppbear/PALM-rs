// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let entries = (1..=1000).map(|i| Bucket { hash: HashValue::default(), key: i, value: i * 10 }).collect();
    let iterator = IntoIter::new(entries);
    let slice = iterator.as_slice();
}

#[test]
fn test_as_slice_single_entry() {
    let entries = vec![Bucket { hash: HashValue::default(), key: 1, value: 10 }];
    let iterator = IntoIter::new(entries);
    let slice = iterator.as_slice();
}

#[test]
fn test_as_slice_multiple_entries() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
    ];
    let iterator = IntoIter::new(entries);
    let slice = iterator.as_slice();
}

#[test]
fn test_as_slice_large_entries() {
    let entries = (1..=999).map(|i| Bucket { hash: HashValue::default(), key: i, value: i * 10 }).collect();
    let iterator = IntoIter::new(entries);
    let slice = iterator.as_slice();
}

#[test]
fn test_as_slice_maximal_entries() {
    let entries = (1..=1000).map(|i| Bucket { hash: HashValue::default(), key: i, value: i * 10 }).collect();
    let iterator = IntoIter::new(entries);
    let slice = iterator.as_slice();
}

