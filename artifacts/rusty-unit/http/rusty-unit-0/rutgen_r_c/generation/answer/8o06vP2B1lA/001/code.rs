// Answer 0

#[test]
fn test_index_valid() {
    struct TestBucket {
        hash: u64,
        key: HeaderName,
        value: String,
        links: Option<Links>,
    }
    
    let buckets = vec![
        TestBucket {
            hash: 1,
            key: HeaderName::from_str("header1").unwrap(),
            value: String::from("value1"),
            links: Some(Links { next: 1, tail: 2 }),
        },
        TestBucket {
            hash: 2,
            key: HeaderName::from_str("header2").unwrap(),
            value: String::from("value2"),
            links: Some(Links { next: 2, tail: 3 }),
        },
    ];

    let raw_links = RawLinks(Box::into_raw(buckets.into_boxed_slice()));
    
    // Access the first bucket's links
    let result = raw_links.index(0);
    assert!(result.is_some());
    assert_eq!(result.unwrap().next, 1);
    assert_eq!(result.unwrap().tail, 2);
    
    // Access the second bucket's links
    let result = raw_links.index(1);
    assert!(result.is_some());
    assert_eq!(result.unwrap().next, 2);
    assert_eq!(result.unwrap().tail, 3);
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    struct TestBucket {
        hash: u64,
        key: HeaderName,
        value: String,
        links: Option<Links>,
    }
    
    let buckets = vec![
        TestBucket {
            hash: 1,
            key: HeaderName::from_str("header1").unwrap(),
            value: String::from("value1"),
            links: Some(Links { next: 1, tail: 2 }),
        },
    ];

    let raw_links = RawLinks(Box::into_raw(buckets.into_boxed_slice()));

    // Access an out-of-bounds index
    let _ = raw_links.index(2);
}

#[test]
#[should_panic]
fn test_index_negative() {
    struct TestBucket {
        hash: u64,
        key: HeaderName,
        value: String,
        links: Option<Links>,
    }
    
    let buckets = vec![
        TestBucket {
            hash: 1,
            key: HeaderName::from_str("header1").unwrap(),
            value: String::from("value1"),
            links: Some(Links { next: 1, tail: 2 }),
        },
    ];

    let raw_links = RawLinks(Box::into_raw(buckets.into_boxed_slice()));

    // Access a negative index (invalid in Rust, will still panic)
    let _ = raw_links.index(usize::MAX);
}

