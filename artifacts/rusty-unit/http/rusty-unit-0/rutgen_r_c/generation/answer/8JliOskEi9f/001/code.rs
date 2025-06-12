// Answer 0

#[test]
fn test_index_mut_valid_index() {
    struct TestRawLinks {
        data: RawLinks<usize>,
    }

    let mut buckets = [Bucket {
        hash: HashValue::default(),
        key: HeaderName::new("test").unwrap(),
        value: 42,
        links: Some(Links { next: 1, tail: 2 }),
    }; 10]; // Create an array of 10 Buckets

    let raw_links = RawLinks(&mut buckets);

    let mut test_raw_links = TestRawLinks { data: raw_links };

    // Valid index within bounds
    let idx: usize = 0;
    let links = test_raw_links.data.index_mut(idx);
    assert!(links.is_some());
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds() {
    struct TestRawLinks {
        data: RawLinks<usize>,
    }

    let mut buckets = [Bucket {
        hash: HashValue::default(),
        key: HeaderName::new("test").unwrap(),
        value: 42,
        links: Some(Links { next: 1, tail: 2 }),
    }; 10]; // Create an array of 10 Buckets

    let raw_links = RawLinks(&mut buckets);

    let mut test_raw_links = TestRawLinks { data: raw_links };

    // Out of bounds index
    let idx: usize = 10; // Index is beyond the valid range
    test_raw_links.data.index_mut(idx);
}

#[test]
fn test_index_mut_modify_links() {
    struct TestRawLinks {
        data: RawLinks<usize>,
    }

    let mut buckets = [Bucket {
        hash: HashValue::default(),
        key: HeaderName::new("test").unwrap(),
        value: 42,
        links: Some(Links { next: 1, tail: 2 }),
    }; 5]; // Create an array of 5 Buckets

    let raw_links = RawLinks(&mut buckets);

    let mut test_raw_links = TestRawLinks { data: raw_links };

    let idx: usize = 2; // Valid index
    let links = test_raw_links.data.index_mut(idx);
    
    // Modify the links
    links.next = 3;
    links.tail = 4;

    // Verify the modification
    assert_eq!(test_raw_links.data.index_mut(idx).next, 3);
    assert_eq!(test_raw_links.data.index_mut(idx).tail, 4);
}

