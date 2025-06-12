// Answer 0

#[test]
fn test_index_within_bounds_lower() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let keys = Keys { iter: buckets.iter() };
    let result = keys.index(0);
}

#[test]
fn test_index_within_bounds_middle() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let keys = Keys { iter: buckets.iter() };
    let result = keys.index(1);
}

#[test]
fn test_index_within_bounds_upper() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let keys = Keys { iter: buckets.iter() };
    let result = keys.index(2);
}

#[should_panic]
#[test]
fn test_index_out_of_bounds_upper() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let keys = Keys { iter: buckets.iter() };
    let result = keys.index(3);
}

#[should_panic]
#[test]
fn test_index_out_of_bounds_lower() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let keys = Keys { iter: buckets.iter() };
    let result = keys.index(usize::MAX);
}

