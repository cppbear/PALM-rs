// Answer 0

#[test]
fn test_value_iter_mut_with_valid_index() {
    #[derive(Clone)]
    struct TestValue;

    let mut header_map = HeaderMap::with_capacity(10);
    let index = 0;

    header_map.entries.push(Bucket {
        hash: 0,
        key: HeaderName::from("key1"),
        value: TestValue,
        links: None,
    });

    let iter = header_map.value_iter_mut(index);
    assert!(iter.index == index);
    assert!(iter.front == Some(Cursor::Head));
}

#[test]
fn test_value_iter_mut_with_empty_links() {
    #[derive(Clone)]
    struct TestValue;

    let mut header_map = HeaderMap::with_capacity(10);
    let index = 0;

    header_map.entries.push(Bucket {
        hash: 0,
        key: HeaderName::from("key1"),
        value: TestValue,
        links: None,
    });

    let iter = header_map.value_iter_mut(index);
    assert!(iter.front == Some(Cursor::Head));
    assert!(iter.back == Some(Cursor::Head));
}

#[test]
fn test_value_iter_mut_with_linked_entries() {
    #[derive(Clone)]
    struct TestValue;

    let mut header_map = HeaderMap::with_capacity(10);
    let index = 0;

    header_map.entries.push(Bucket {
        hash: 0,
        key: HeaderName::from("key1"),
        value: TestValue,
        links: Some(Links { next: 1, tail: 1 }),
    });

    header_map.entries.push(Bucket {
        hash: 0,
        key: HeaderName::from("key2"),
        value: TestValue,
        links: None,
    });

    let iter = header_map.value_iter_mut(index);
    assert!(iter.index == index);
    assert!(iter.front == Some(Cursor::Head));
    assert!(matches!(iter.back, Some(Cursor::Values(1))));
}

#[test]
#[should_panic]
fn test_value_iter_mut_with_out_of_bounds_index() {
    #[derive(Clone)]
    struct TestValue;

    let mut header_map = HeaderMap::with_capacity(10);
    let index = 1; // Accessing an index that does not exist

    header_map.entries.push(Bucket {
        hash: 0,
        key: HeaderName::from("key1"),
        value: TestValue,
        links: None,
    });

    let _iter = header_map.value_iter_mut(index); // This should panic
}

