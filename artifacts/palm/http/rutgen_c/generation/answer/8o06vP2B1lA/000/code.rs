// Answer 0

#[test]
fn test_index_valid_index() {
    #[derive(Debug)]
    struct TestBucket {
        hash: HashValue,
        key: HeaderName,
        value: HeaderValue,
        links: Option<Links>,
    }

    let bucket_array = vec![
        TestBucket {
            hash: HashValue::default(),
            key: HeaderName::new("test1").unwrap(),
            value: HeaderValue::from("value1"),
            links: Some(Links { next: 1, tail: 2 }),
        },
        TestBucket {
            hash: HashValue::default(),
            key: HeaderName::new("test2").unwrap(),
            value: HeaderValue::from("value2"),
            links: None,
        },
    ];
    
    let raw_links = RawLinks(bucket_array.as_mut_ptr());
    let index = raw_links.index(0);
    
    assert!(index.is_some());
    assert_eq!(index.unwrap(), &Some(Links { next: 1, tail: 2 }));
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    #[derive(Debug)]
    struct TestBucket {
        hash: HashValue,
        key: HeaderName,
        value: HeaderValue,
        links: Option<Links>,
    }

    let bucket_array = vec![
        TestBucket {
            hash: HashValue::default(),
            key: HeaderName::new("test1").unwrap(),
            value: HeaderValue::from("value1"),
            links: Some(Links { next: 1, tail: 2 }),
        }
    ];
    
    let raw_links = RawLinks(bucket_array.as_mut_ptr());
    let _ = raw_links.index(1); // This should panic due to out of bounds access
}

