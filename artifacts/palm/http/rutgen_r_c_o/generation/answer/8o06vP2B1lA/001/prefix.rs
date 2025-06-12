// Answer 0

#[test]
fn test_index_first_element() {
    let bucket = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from("Test-Header"),
        value: HeaderValue::from("Test-Value"),
        links: Some(Links { next: 1, tail: 1 }),
    };
    let raw_links = RawLinks(Box::into_raw(Box::new([bucket; 1])));
    let _result = raw_links.index(0);
}

#[test]
fn test_index_middle_element() {
    let bucket1 = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from("Header-1"),
        value: HeaderValue::from("Value-1"),
        links: Some(Links { next: 1, tail: 1 }),
    };
    let bucket2 = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from("Header-2"),
        value: HeaderValue::from("Value-2"),
        links: Some(Links { next: 2, tail: 2 }),
    };
    let raw_links = RawLinks(Box::into_raw(Box::new([bucket1, bucket2])));
    let _result = raw_links.index(1);
}

#[test]
fn test_index_last_element() {
    let bucket = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from("Last-Header"),
        value: HeaderValue::from("Last-Value"),
        links: Some(Links { next: 0, tail: 0 }),
    };
    let raw_links = RawLinks(Box::into_raw(Box::new([bucket; 1])));
    let _result = raw_links.index(0);
}

#[should_panic]
fn test_index_out_of_bounds() {
    let bucket = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from("Out-Of-Bounds"),
        value: HeaderValue::from("Value"),
        links: None,
    };
    let raw_links = RawLinks(Box::into_raw(Box::new([bucket; 1])));
    let _result = raw_links.index(1);
}

