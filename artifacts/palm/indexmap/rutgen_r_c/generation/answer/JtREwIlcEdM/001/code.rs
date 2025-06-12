// Answer 0

#[test]
fn test_fmt_empty_iter() {
    struct TestBucket {
        hash: usize,
        key: String,
        value: String,
    }

    let buckets: Vec<TestBucket> = Vec::new();
    let slice: &[Bucket<TestBucket>] = &buckets.iter()
        .map(|b| Bucket {
            hash: b.hash,
            key: b.key.clone(),
            value: b.value.clone(),
        }).collect::<Vec<_>>(); 

    let iter = Iter { iter: slice.iter() };

    let result = format!("{:?}", iter);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_single_element() {
    struct TestBucket {
        hash: usize,
        key: String,
        value: String,
    }

    let buckets = vec![TestBucket {
        hash: 1,
        key: String::from("test_key"),
        value: String::from("test_value"),
    }];
    
    let slice: &[Bucket<TestBucket>] = &buckets.iter()
        .map(|b| Bucket {
            hash: b.hash,
            key: b.key.clone(),
            value: b.value.clone(),
        }).collect::<Vec<_>>();

    let iter = Iter { iter: slice.iter() };

    let result = format!("{:?}", iter);
    assert_eq!(result, "[test_key: test_value]"); // Expected format output may vary based on actual implementation
}

#[test]
fn test_fmt_multiple_elements() {
    struct TestBucket {
        hash: usize,
        key: String,
        value: String,
    }

    let buckets = vec![
        TestBucket {
            hash: 1,
            key: String::from("first_key"),
            value: String::from("first_value"),
        },
        TestBucket {
            hash: 2,
            key: String::from("second_key"),
            value: String::from("second_value"),
        },
    ];
    
    let slice: &[Bucket<TestBucket>] = &buckets.iter()
        .map(|b| Bucket {
            hash: b.hash,
            key: b.key.clone(),
            value: b.value.clone(),
        }).collect::<Vec<_>>();

    let iter = Iter { iter: slice.iter() };

    let result = format!("{:?}", iter);
    assert_eq!(result, "[first_key: first_value, second_key: second_value]"); // Expected format output may vary based on actual implementation
}

