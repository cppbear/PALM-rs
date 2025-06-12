// Answer 0

#[test]
fn test_index_mut_valid_index() {
    let mut bucket = Bucket {
        hash: HashValue::default(),
        key: HeaderName::default(),
        value: HeaderValue::default(),
        links: Some(Links { next: 1, tail: 2 }),
    };
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([bucket])));
    
    let index = 0;
    let links = raw_links.index_mut(index);
    assert!(links.is_some());
    assert_eq!(links.next, 1);
    assert_eq!(links.tail, 2);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds() {
    let mut bucket = Bucket {
        hash: HashValue::default(),
        key: HeaderName::default(),
        value: HeaderValue::default(),
        links: Some(Links { next: 1, tail: 2 }),
    };

    let mut raw_links = RawLinks(Box::into_raw(Box::new([bucket])));
    
    let index = 1; // out of bounds
    let _links = raw_links.index_mut(index); // This should panic
}

#[test]
fn test_index_mut_links_none() {
    let mut bucket = Bucket {
        hash: HashValue::default(),
        key: HeaderName::default(),
        value: HeaderValue::default(),
        links: None,
    };
    
    let mut raw_links = RawLinks(Box::into_raw(Box::new([bucket])));
    
    let index = 0;
    let links = raw_links.index_mut(index);
    assert!(links.is_none());
}

