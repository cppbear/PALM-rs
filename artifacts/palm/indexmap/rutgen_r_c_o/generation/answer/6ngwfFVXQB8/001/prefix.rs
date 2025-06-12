// Answer 0

#[test]
fn test_first_mut_empty() {
    let mut slice = Slice::new_mut();
    let result = slice.first_mut();
}

#[test]
fn test_first_mut_single_entry() {
    let buckets = vec![Bucket { hash: HashValue::default(), key: 1, value: 100 }];
    let mut slice = Slice { entries: buckets };
    let result = slice.first_mut();
}

#[test]
fn test_first_mut_multiple_entries() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: 100 },
        Bucket { hash: HashValue::default(), key: 2, value: 200 },
    ];
    let mut slice = Slice { entries: buckets };
    let result = slice.first_mut();
}

